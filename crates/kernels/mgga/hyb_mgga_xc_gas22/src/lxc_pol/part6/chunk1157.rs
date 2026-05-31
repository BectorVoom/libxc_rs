//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1157/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1157<F: Float>(t19746: F, t35: F, t40: F, t17: F, t1802: F, t573: F, t1896: F, t50: F, t68: F, t6091: F, t78: F, t1952: F) -> (F, F, F, F, F, F) {
    let t19749 = F::cast_from(140.0_f64) / F::cast_from(729.0_f64) * t35 * t19746 * t40;
    let t19754 = t17 / t573 / t1802;
    let t19755 = t1896 * t1896;
    let t19756 = F::cast_from(1.0_f64) / t19755;
    let t19824 = F::cast_from(1.0_f64) / t68 / t50;
    let t19952 = F::cast_from(1.0_f64) / t6091 / t78;
    let t19960 = F::cast_from(1.0_f64) / t6091 / t1952;
    (t19749, t19754, t19756, t19824, t19952, t19960)
}
