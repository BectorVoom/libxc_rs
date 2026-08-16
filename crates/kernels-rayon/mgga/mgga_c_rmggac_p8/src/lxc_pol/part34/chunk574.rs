//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 574/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk574(t14650: f64, t14156: f64, t14168: f64, t14171: f64, t14175: f64, t14186: f64, t14190: f64, t14194: f64, t14200: f64, t14202: f64, t3219: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14651 = 0.39914139006212695214e-1_f64 * t14650;
    let t14653 = 0.10227998120342003148e-1_f64 * t14156;
    let t14654 = 0.58171619854173713846e-5_f64 * t14168;
    let t14655 = 0.44903406381989282115e-1_f64 * t14171;
    let t14656 = 0.14967802127329760705e-1_f64 * t14175;
    let t14659 = 0.85129199786595678799e-5_f64 * t14186;
    let t14660 = 0.2553875993597870364e-4_f64 * t14190;
    let t14661 = 0.2553875993597870364e-4_f64 * t14194;
    let t14662 = 0.1702583995731913576e-4_f64 * t14200;
    let t14663 = 0.85129199786595678799e-5_f64 * t14202;
    let t14664 = t7720 * t3219;
    (t14651, t14653, t14654, t14655, t14656, t14659, t14660, t14661, t14662, t14663, t14664)
}
