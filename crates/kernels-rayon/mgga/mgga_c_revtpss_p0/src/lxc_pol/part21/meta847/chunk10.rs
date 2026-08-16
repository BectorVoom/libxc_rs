//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3185/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3185(t12571: f64, t5207: f64, t12486: f64, t300: f64, t1187: f64, t3515: f64, t5184: f64, t16812: f64, t3531: f64, t12553: f64, t16997: f64, t1196: f64, t16672: f64, t3498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58664 = 0.51947577317044391277e2_f64 * t12571 * t5207;
    let t58665 = t300 * t12486;
    let t58666 = t3515 * t1187;
    let t58669 = 0.31168546390226634765e3_f64 * t58665 * t5184 * t58666;
    let t58671 = 0.30762056574649219973e4_f64 * t3531 * t16812;
    let t58672 = t300 * t12553;
    let t58675 = 0.30762056574649219974e4_f64 * t58672 * t16997 * t58666;
    let t58678 = 0.10526802520742363173e2_f64 * t1196 * t16672 * t3498;
    (t58664, t58666, t58669, t58671, t58675, t58678)
}
