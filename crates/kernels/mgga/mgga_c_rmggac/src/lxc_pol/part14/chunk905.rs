//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 905/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk905<F: Float>(t4905: F, t8946: F, t36284: F, t36286: F, t39700: F, t797: F, t40897: F, t5271: F, t2376: F, t27048: F, t27176: F, t36269: F, t36272: F, t36278: F, t36294: F, t40725: F, t5245: F, t866: F, t8936: F, t8940: F) -> (F, F) {
    let t41518 = t8946 * t4905;
    let t41521 = 0.5854073720911195298e0 * t36284;
    let t41522 = 0.8781110581366792947e0 * t36286;
    let t41523 = t797 * t39700;
    let t41524 = 0.23948483403727617128e0 * t41523;
    let t41531 = t5271 * t40897;
    let t41532 = 0.47896966807455234256e0 * t41531;
    let t41533 = 0.71845450211182851384e0 * t27048 * t40725 - 0.21819729323396273384e0 * t36269 - 0.54549323308490683458e-1 * t36272 + 0.72732431077987577944e-1 * t36278 - 0.95793933614910468512e0 * t27176 * t41518 + t41521 - t41522 + t41524 + 0.11974241701863808564e0 * t5245 * t2376 - 0.79828278012425390426e-1 * t36294 + 0.11974241701863808564e0 * t8940 * t8936 * t866 + t41532;
    (t41518, t41533)
}
