//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 669/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk669(t8716: f64, t8718: f64, t8735: f64, t8737: f64, t8741: f64, t8832: f64, t8837: f64, t8844: f64, t8846: f64, t8872: f64, t9001: f64, t9009: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9447 = 0.18183107769496894486e-1_f64 * t8716;
    let t9448 = 0.24244143692662525982e-1_f64 * t8718;
    let t9457 = 0.17701538806747441785e-2_f64 * t8735;
    let t9458 = 0.21241846568096930142e-2_f64 * t8737;
    let t9460 = 0.53218852008283593619e-1_f64 * t8741;
    let t9490 = 0.3192344991997337955e-4_f64 * t8832;
    let t9491 = 0.3192344991997337955e-4_f64 * t8837;
    let t9492 = 0.1064114997332445985e-4_f64 * t8844;
    let t9493 = 0.1064114997332445985e-4_f64 * t8846;
    let t9501 = 0.8980681276397856423e-1_f64 * t8872;
    let t9583 = 0.15965655602485078085e0_f64 * t9001;
    let t9586 = 0.23948483403727617128e0_f64 * t9009;
    (t9447, t9448, t9457, t9458, t9460, t9490, t9491, t9492, t9493, t9501, t9583, t9586)
}
