//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1546/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546(t24407: f64, t3520: f64, t24294: f64, t698: f64, t24288: f64, t24291: f64, t24274: f64, t24271: f64, t24312: f64, t3390: f64, t24297: f64, t24323: f64, t3435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81310 = t3520 * t24407;
    let t81425 = t698 * t24294;
    let t81427 = t698 * t24288;
    let t81429 = t698 * t24291;
    let t81491 = t698 * t24274;
    let t81496 = t698 * t24271;
    let t81513 = t3390 * t24312;
    let t81539 = t698 * t24297;
    let t81650 = t24323 * t3435;
    (t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650)
}
