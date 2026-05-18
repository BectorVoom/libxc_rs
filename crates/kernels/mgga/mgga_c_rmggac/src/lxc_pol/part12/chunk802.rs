//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 802/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk802<F: Float>(t8400: F, t8418: F, t8423: F, t8438: F, t8444: F, t8448: F, t8452: F, t8460: F, t34544: F, t34545: F, t34548: F, t7303: F, t7307: F, t9282: F) -> (F, F, F, F) {
    let t38211 = F::new(0.23948483403727617128e0) * t8400;
    let t38212 = F::new(0.17025839957319135759e-4) * t8418;
    let t38213 = F::new(0.85129199786595678796e-5) * t8423;
    let t38217 = F::new(0.85129199786595678796e-5) * t8438;
    let t38218 = F::new(0.85129199786595678796e-5) * t8444;
    let t38219 = F::new(0.85129199786595678796e-5) * t8448;
    let t38220 = F::new(0.85129199786595678796e-5) * t8452;
    let t38221 = F::new(0.39914139006212695214e-1) * t8460;
    let t38224 = t38217 - t38218 - t38219 - t38220 + t9282 - t38221 + t34544 - t34545 - F::new(0.60975299583150056628e-3) * t7303 - F::new(0.60975299583150056628e-3) * t7307 + t34548;
    (t38211, t38212, t38213, t38224)
}
