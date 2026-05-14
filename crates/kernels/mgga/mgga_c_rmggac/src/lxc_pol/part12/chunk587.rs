//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 587/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk587<F: Float>(t7438: F, t7708: F, t7770: F, t7780: F, t7908: F, t7910: F, t7940: F, t2416: F, t7487: F, t2160: F, t2339: F, t638: F, t2323: F, t1540: F, t511: F, t650: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8094 = 0.11918087970123395032e-3 * t7438;
    let t8173 = 0.3193131120497015617e0 * t7708;
    let t8196 = 0.47896966807455234256e0 * t7770;
    let t8197 = 0.15965655602485078085e0 * t7780;
    let t8221 = 0.39726959900411316772e-4 * t7908;
    let t8222 = 0.11918087970123395032e-3 * t7910;
    let t8304 = 0.39726959900411316772e-4 * t7940;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    let t8339 = t1540 * t511;
    let t8340 = t8339 * t650;
    (t8094, t8173, t8196, t8197, t8221, t8222, t8304, t8328, t8331, t8334, t8339, t8340)
}
