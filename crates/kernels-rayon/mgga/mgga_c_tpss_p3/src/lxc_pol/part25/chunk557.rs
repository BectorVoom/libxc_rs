//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 557/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk557(t2761: f64, t66: f64, t219: f64, t976: f64, t371: f64, t979: f64, t73: f64, t2711: f64, t2712: f64, t2715: f64, t329: f64, t356: f64) -> (f64, f64, f64, f64, f64) {
    let t2762 = t66 * t2761;
    let t2771 = t976 * t219;
    let t2775 = 1.0_f64 / t979 / t371;
    let t2776 = t73 * t2775;
    let t2782 = t2711 * t2712 * t2715;
    let t2785 = 1.0_f64 / t356 / t329;
    (t2762, t2771, t2776, t2782, t2785)
}
