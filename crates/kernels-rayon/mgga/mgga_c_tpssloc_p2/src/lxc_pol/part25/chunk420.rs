//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 420/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk420(t5: f64, t2233: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t605: f64, t645: f64, t86: f64, t112: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t2311 = piecewise3(t8, 0.0_f64, t2233 * t86 - 8.0_f64 * t2235 * t645 + 20.0_f64 * t2240 * t2241 - 4.0_f64 * t2307 * t605);
    let t2312 = t2311 * t112;
    (t2311, t2312)
}
