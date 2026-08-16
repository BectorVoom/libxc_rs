//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1308/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1308(t10350: f64, t10424: f64, t1270: f64, t1282: f64, t172: f64, t180: f64, t184: f64, t2104: f64, t2111: f64, t2116: f64, t2144: f64, t24320: f64, t28459: f64, t28476: f64, t28505: f64, t28538: f64, t28576: f64, t3227: f64, t3235: f64, t3252: f64, t3264: f64, t4046: f64, t4079: f64, t6363: f64, t740: f64, t742: f64, t756: f64, t8354: f64, t8431: f64) -> f64 {
    let t28585 = 14.0_f64 * t3252 * t28476 - t24320 * t28476 - 24.0_f64 * t6363 * t3235 * t3227 + 2.0_f64 * t2104 * t4079 + 4.0_f64 * t740 * t10424 + 2.0_f64 * t4046 * t2144 + 4.0_f64 * t8354 * t1282 + 8.0_f64 * t3227 * t3264 + 4.0_f64 * t1270 * t8431 + 2.0_f64 * t28459 * t184 + 4.0_f64 * t10350 * t756 + 2.0_f64 * t172 * (t28538 + t28576) - t742 * t28459 - t2111 * t28505 * t180 + 4.0_f64 * t2116 * t28505;
    t28585
}
