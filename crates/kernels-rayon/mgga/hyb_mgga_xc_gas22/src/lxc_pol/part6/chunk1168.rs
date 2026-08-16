//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1168/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1168(t20624: f64, t2288: f64, t2310: f64, t2313: f64, t6561: f64, t783: f64, t2232: f64, t230: f64, t2235: f64, t2180: f64, t2233: f64, t2187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20714 = 280.0_f64 / 81.0_f64 * t20624;
    let t20730 = 1.0_f64 / t2310 / t2288;
    let t20740 = t2310 * t2310;
    let t20741 = 1.0_f64 / t20740;
    let t20743 = t2313 * t2313;
    let t20744 = 1.0_f64 / t20743;
    let t20770 = 0.18467901234567901234e0_f64 * t20624;
    let t20824 = t783 * t6561;
    let t20827 = t2232 * t2232;
    let t20829 = t230 / t20827;
    let t20831 = t2235 * t2235;
    let t20832 = 1.0_f64 / t20831;
    let t20838 = t2180 * t2233;
    let t20843 = t2180 * t2187;
    (t20714, t20730, t20741, t20744, t20770, t20824, t20829, t20832, t20838, t20843)
}
