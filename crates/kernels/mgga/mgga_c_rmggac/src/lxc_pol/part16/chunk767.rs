//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 767/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk767<F: Float>(t8353: F, t8359: F, t8363: F, t8366: F, t8369: F, t8405: F, t8408: F, t8411: F, t8414: F, t8418: F, t8423: F, t8428: F, t8433: F, t8438: F, t8444: F, t8448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42374 = 0.1440846329149835838e-2 * t8353;
    let t42375 = 0.1440846329149835838e-2 * t8359;
    let t42376 = 0.1440846329149835838e-2 * t8363;
    let t42377 = 0.5454932330849068346e-1 * t8366;
    let t42378 = 0.13637330827122670865e-1 * t8369;
    let t42383 = 0.11974241701863808564e0 * t8405;
    let t42384 = 0.17961362552795712846e0 * t8408;
    let t42385 = 0.35922725105591425692e0 * t8411;
    let t42386 = 0.11974241701863808564e0 * t8414;
    let t42390 = 0.3405167991463827152e-4 * t8418;
    let t42391 = 0.1702583995731913576e-4 * t8423;
    let t42392 = 0.5107751987195740728e-4 * t8428;
    let t42393 = 0.5107751987195740728e-4 * t8433;
    let t42394 = 0.1702583995731913576e-4 * t8438;
    let t42395 = 0.1702583995731913576e-4 * t8444;
    let t42396 = 0.1702583995731913576e-4 * t8448;
    (t42374, t42375, t42376, t42377, t42378, t42383, t42384, t42385, t42386, t42390, t42391, t42392, t42393, t42394, t42395, t42396)
}
