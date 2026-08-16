//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 781/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk781(t7767: f64, t8201: f64, t7901: f64, t34687: f64, t34704: f64, t34706: f64, t34710: f64, t34752: f64, t34772: f64, t34784: f64, t34787: f64, t34793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37179 = 0.18292589874945016987e-2_f64 * t7767;
    let t37183 = 3.0_f64 * t8201;
    let t37186 = 0.87811105813667929468e0_f64 * t7901;
    let t37200 = 0.205201155180140685e-5_f64 * t34687;
    let t37201 = 0.18292589874945016987e-2_f64 * t34704;
    let t37202 = 0.91462949374725084936e-3_f64 * t34706;
    let t37203 = 0.13010691197123848592e-3_f64 * t34710;
    let t37214 = 0.205201155180140685e-5_f64 * t34752;
    let t37218 = 0.30487649791575028312e-3_f64 * t34772;
    let t37221 = 0.91462949374725084936e-3_f64 * t34784;
    let t37222 = 0.13010691197123848592e-3_f64 * t34787;
    let t37223 = 0.18292589874945016987e-2_f64 * t34793;
    (t37179, t37183, t37186, t37200, t37201, t37202, t37203, t37214, t37218, t37221, t37222, t37223)
}
