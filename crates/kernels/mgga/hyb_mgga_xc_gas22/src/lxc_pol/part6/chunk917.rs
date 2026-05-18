//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 917/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk917<F: Float>(t29: F, t8145: F, t125: F, t26: F, t3114: F, t550: F, t19: F, t3118: F, t2950: F, t641: F, t669: F, t1181: F, t1233: F, t1824: F, t1863: F, t1867: F, t1997: F, t2949: F, t3115: F, t3119: F, t547: F, t555: F, t558: F, t6162: F, t7921: F, t7925: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8146 = t29 * t8145;
    let t8147 = t8146 * t125;
    let t8148 = t26 * t8147;
    let t8157 = t550 * t3114;
    let t8159 = t19 * t8157 / F::new(32.0);
    let t8160 = t550 * t3118;
    let t8162 = t19 * t8160 / F::new(32.0);
    let t8169 = t2950 * t641;
    let t8172 = t2950 * t669;
    let t8176 = -t555 * t558 * t7921 / F::new(32.0) - t555 * t558 * t7925 / F::new(64.0) - F::new(3.0) / F::new(64.0) * t19 * t8148 - F::new(3.0) / F::new(64.0) * t1867 * t1233 - F::new(3.0) / F::new(32.0) * t547 * t3115 - F::new(3.0) / F::new(32.0) * t547 * t3119 - t8159 - t8162 - F::new(3.0) / F::new(64.0) * t1181 * t1997 - F::new(3.0) / F::new(32.0) * t1181 * t1824 - F::new(3.0) / F::new(64.0) * t1181 * t1863 - F::new(3.0) / F::new(16.0) * t2949 * t8169 - F::new(3.0) / F::new(16.0) * t2949 * t8172 + t6162 / F::new(144.0);
    (t8147, t8148, t8157, t8159, t8160, t8162, t8169, t8172, t8176)
}
