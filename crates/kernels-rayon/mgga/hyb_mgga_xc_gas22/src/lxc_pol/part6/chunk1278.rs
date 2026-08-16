//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1278/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1278(t10132: f64, t1819: f64, t555: f64, t1179: f64, t1181: f64, t125: f64, t1796: f64, t1804: f64, t1807: f64, t19: f64, t23023: f64, t23050: f64, t23767: f64, t23772: f64, t26: f64, t27099: f64, t27102: f64, t27105: f64, t27120: f64, t27612: f64, t29: f64, t2949: f64, t2950: f64, t2970: f64, t2972: f64, t2987: f64, t3: f64, t3118: f64, t3814: f64, t545: f64, t558: f64, t6164: f64, t6190: f64, t6195: f64, t7913: f64, t8205: f64, t9833: f64, t9909: f64) -> f64 {
    let t27624 = t555 * t1819 * t10132;
    let t27635 = -t27099 / 96.0_f64 - t27102 / 96.0_f64 - t27105 / 72.0_f64 - t555 * t558 * t23023 * t1179 / 32.0_f64 - t1804 * t1807 * t6190 * t3814 / 48.0_f64 - t1804 * t1807 * t6195 * t3814 / 24.0_f64 + t27120 / 144.0_f64 - t1804 * t1807 * t6164 * t3814 / 48.0_f64 - t555 * t558 * t23767 * t1179 / 32.0_f64 - t555 * t558 * t23772 * t1179 / 16.0_f64 - t555 * t2987 * t7913 * t3 / 8.0_f64 - t23050 / 18.0_f64 - 3.0_f64 / 64.0_f64 * t19 * t26 * t29 * t27612 * t125 - 3.0_f64 / 32.0_f64 * t1181 * t8205 - 3.0_f64 / 8.0_f64 * t2949 * t2950 * t3118 - t27624 / 96.0_f64 - t2970 * t2972 * t125 * t9909 * t545 / 24.0_f64 - t2970 * t2972 * t9833 * t1796 / 48.0_f64;
    t27635
}
