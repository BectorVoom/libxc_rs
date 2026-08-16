//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1515/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1515(t10678: f64, t10682: f64, t10687: f64, t10692: f64, t14759: f64, t14761: f64, t14765: f64, t14769: f64, t14774: f64, t14777: f64, t14780: f64, t14783: f64, t851: f64) -> f64 {
    let t14784 = t14759 - 0.45178982497454656791e-5_f64 * t14761 - 0.60976381323476959249e-3_f64 * t10678 + 0.28582678745379824648e-4_f64 * t10682 - t10687 + t10692 - 35.0_f64 / 216.0_f64 * t14765 + 0.42874018118069736972e-2_f64 * t851 * t14769 - 0.25724410870841842183e-1_f64 * t851 * t14774 - 0.80031500487063509015e-2_f64 * t14777 + 0.10164000561857065645e-4_f64 * t14780 + t14783;
    t14784
}
