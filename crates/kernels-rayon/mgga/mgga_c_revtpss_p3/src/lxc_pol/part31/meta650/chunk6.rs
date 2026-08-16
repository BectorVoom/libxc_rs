//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2150/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150(t100008: f64, t100138: f64, t100141: f64, t100186: f64, t19682: f64, t19688: f64, t19693: f64, t19707: f64, t19722: f64, t19750: f64, t19754: f64, t19758: f64, t19792: f64, t25522: f64, t6273: f64, t7132: f64, t93548: f64, t93670: f64, t99985: f64) -> f64 {
    let t107012 = 0.42874018118069736972e-3_f64 * t99985 * t19722 + 0.25724410870841842183e-2_f64 * t100138 * t19750 - 0.25724410870841842183e-2_f64 * t100141 * t19754 + 0.42874018118069736972e-3_f64 * t93548 * t19758 + 0.45732285992607719437e-2_f64 * t93670 * t6273 - t100186 + 0.11433071498151929859e-2_f64 * t100008 * t19707 - 0.57165357490759649296e-3_f64 * t25522 * t19792 - 0.57165357490759649296e-3_f64 * t7132 * t19682 + 0.47637797908966374413e-3_f64 * t7132 * t19688 - 0.47637797908966374413e-3_f64 * t25522 * t19693;
    t107012
}
