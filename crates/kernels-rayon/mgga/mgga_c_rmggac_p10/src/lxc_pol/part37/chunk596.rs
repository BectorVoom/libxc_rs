//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 596/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk596(t3076: f64, t570: f64, t2044: f64, t13839: f64, t234: f64, t551: f64, t3157: f64, t3167: f64, t8368: f64, t2367: f64, t649: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15271 = t3076 * t570;
    let t15272 = t2044 * t15271;
    let t15273 = t13839 * t15272;
    let t15280 = t234 * t551;
    let t15281 = t15280 * t3157;
    let t15284 = t8368 * t3167;
    let t15286 = t649 * t2367;
    let t15287 = t27 * t15286;
    (t15272, t15273, t15280, t15281, t15284, t15287)
}
