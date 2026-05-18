//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1059/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1059<F: Float>(t1707: F, t2064: F, t3928: F, t1550: F, t6522: F, t7778: F, t1540: F, t2368: F, t36505: F, t36508: F, t36511: F, t36513: F, t36515: F, t36521: F, t41620: F, t41637: F, t41641: F, t46586: F, t47371: F, t47375: F, t47378: F, t47381: F, t47385: F, t884: F) -> F {
    let t47390 = t3928 * t2064 * t1707;
    let t47393 = t1550 * t7778 * t6522;
    let t47400 = -t41620 - F::new(0.2993560425465952141e-1) * t47371 + F::new(0.59871208509319042821e-1) * t884 * t46586 + F::new(0.19863479950205658386e-4) * t47375 + F::new(0.2993560425465952141e-1) * t47378 - F::new(0.44903406381989282115e-1) * t47381 - F::new(0.36366215538993788972e0) * t41637 + F::new(0.21819729323396273383e0) * t41641 + F::new(0.14967802127329760705e-1) * t47385 + t36505 - F::new(0.39914139006212695214e-1) * t1540 * t2368 - F::new(0.47896966807455234256e0) * t47390 - F::new(0.15965655602485078085e0) * t47393 - F::new(0.33105799917009430643e-4) * t36508 + F::new(0.99317399751028291929e-4) * t36511 - F::new(0.99317399751028291929e-4) * t36513 - F::new(0.33105799917009430643e-4) * t36515 - F::new(0.41382249896261788304e-4) * t36521;
    t47400
}
