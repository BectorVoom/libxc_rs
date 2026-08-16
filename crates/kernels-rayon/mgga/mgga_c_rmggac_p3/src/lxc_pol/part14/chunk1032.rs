//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1032/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1032(t4905: f64, t8946: f64, t36284: f64, t36286: f64, t39700: f64, t797: f64, t40897: f64, t5271: f64, t2376: f64, t27048: f64, t27176: f64, t36269: f64, t36272: f64, t36278: f64, t36294: f64, t40725: f64, t5245: f64, t866: f64, t8936: f64, t8940: f64) -> (f64, f64) {
    let t41518 = t8946 * t4905;
    let t41521 = 0.5854073720911195298e0_f64 * t36284;
    let t41522 = 0.8781110581366792947e0_f64 * t36286;
    let t41523 = t797 * t39700;
    let t41524 = 0.23948483403727617128e0_f64 * t41523;
    let t41531 = t5271 * t40897;
    let t41532 = 0.47896966807455234256e0_f64 * t41531;
    let t41533 = 0.71845450211182851384e0_f64 * t27048 * t40725 - 0.21819729323396273384e0_f64 * t36269 - 0.54549323308490683458e-1_f64 * t36272 + 0.72732431077987577944e-1_f64 * t36278 - 0.95793933614910468512e0_f64 * t27176 * t41518 + t41521 - t41522 + t41524 + 0.11974241701863808564e0_f64 * t5245 * t2376 - 0.79828278012425390426e-1_f64 * t36294 + 0.11974241701863808564e0_f64 * t8940 * t8936 * t866 + t41532;
    (t41518, t41533)
}
