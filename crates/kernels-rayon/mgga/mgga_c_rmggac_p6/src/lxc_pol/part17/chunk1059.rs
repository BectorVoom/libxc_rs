//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1059/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1059(t1707: f64, t2064: f64, t3928: f64, t1550: f64, t6522: f64, t7778: f64, t1540: f64, t2368: f64, t36505: f64, t36508: f64, t36511: f64, t36513: f64, t36515: f64, t36521: f64, t41620: f64, t41637: f64, t41641: f64, t46586: f64, t47371: f64, t47375: f64, t47378: f64, t47381: f64, t47385: f64, t884: f64) -> f64 {
    let t47390 = t3928 * t2064 * t1707;
    let t47393 = t1550 * t7778 * t6522;
    let t47400 = -t41620 - 0.2993560425465952141e-1_f64 * t47371 + 0.59871208509319042821e-1_f64 * t884 * t46586 + 0.19863479950205658386e-4_f64 * t47375 + 0.2993560425465952141e-1_f64 * t47378 - 0.44903406381989282115e-1_f64 * t47381 - 0.36366215538993788972e0_f64 * t41637 + 0.21819729323396273383e0_f64 * t41641 + 0.14967802127329760705e-1_f64 * t47385 + t36505 - 0.39914139006212695214e-1_f64 * t1540 * t2368 - 0.47896966807455234256e0_f64 * t47390 - 0.15965655602485078085e0_f64 * t47393 - 0.33105799917009430643e-4_f64 * t36508 + 0.99317399751028291929e-4_f64 * t36511 - 0.99317399751028291929e-4_f64 * t36513 - 0.33105799917009430643e-4_f64 * t36515 - 0.41382249896261788304e-4_f64 * t36521;
    t47400
}
