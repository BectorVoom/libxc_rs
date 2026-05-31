//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1278/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1278<F: Float>(t10132: F, t1819: F, t555: F, t1179: F, t1181: F, t125: F, t1796: F, t1804: F, t1807: F, t19: F, t23023: F, t23050: F, t23767: F, t23772: F, t26: F, t27099: F, t27102: F, t27105: F, t27120: F, t27612: F, t29: F, t2949: F, t2950: F, t2970: F, t2972: F, t2987: F, t3: F, t3118: F, t3814: F, t545: F, t558: F, t6164: F, t6190: F, t6195: F, t7913: F, t8205: F, t9833: F, t9909: F) -> F {
    let t27624 = t555 * t1819 * t10132;
    let t27635 = -t27099 / F::cast_from(96.0_f64) - t27102 / F::cast_from(96.0_f64) - t27105 / F::cast_from(72.0_f64) - t555 * t558 * t23023 * t1179 / F::cast_from(32.0_f64) - t1804 * t1807 * t6190 * t3814 / F::cast_from(48.0_f64) - t1804 * t1807 * t6195 * t3814 / F::cast_from(24.0_f64) + t27120 / F::cast_from(144.0_f64) - t1804 * t1807 * t6164 * t3814 / F::cast_from(48.0_f64) - t555 * t558 * t23767 * t1179 / F::cast_from(32.0_f64) - t555 * t558 * t23772 * t1179 / F::cast_from(16.0_f64) - t555 * t2987 * t7913 * t3 / F::cast_from(8.0_f64) - t23050 / F::cast_from(18.0_f64) - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t19 * t26 * t29 * t27612 * t125 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t1181 * t8205 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2949 * t2950 * t3118 - t27624 / F::cast_from(96.0_f64) - t2970 * t2972 * t125 * t9909 * t545 / F::cast_from(24.0_f64) - t2970 * t2972 * t9833 * t1796 / F::cast_from(48.0_f64);
    t27635
}
