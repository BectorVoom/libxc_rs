//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1497/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497(t23384: f64, t689: f64, t779: f64, t14987: f64, t18797: f64, t23388: f64, t786: f64, t789: f64, t23414: f64, t23413: f64, t41070: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t75950 = t689 * t779 * t23384;
    let t75956 = t14987 * t18797;
    let t75961 = t786 * t23388 * t789;
    let t75974 = t689 * t779 * t23414;
    let t75978 = t41070 * t23413 * t72 * t686;
    (t75950, t75956, t75961, t75974, t75978)
}
