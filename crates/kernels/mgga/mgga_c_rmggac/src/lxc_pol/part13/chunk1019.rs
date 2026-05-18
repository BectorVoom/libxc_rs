//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1019/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1019<F: Float>(t8347: F, t8353: F, t8359: F, t8363: F, t8366: F, t8369: F, t8405: F, t8408: F, t8411: F, t8414: F, t7270: F, t7280: F, t7289: F, t8034: F, t8035: F, t8037: F, t8039: F) -> (F, F, F, F, F, F, F) {
    let t42373 = F::new(0.1440846329149835838e-2) * t8347;
    let t42374 = F::new(0.1440846329149835838e-2) * t8353;
    let t42375 = F::new(0.1440846329149835838e-2) * t8359;
    let t42376 = F::new(0.1440846329149835838e-2) * t8363;
    let t42377 = F::new(0.5454932330849068346e-1) * t8366;
    let t42378 = F::new(0.13637330827122670865e-1) * t8369;
    let t42383 = F::new(0.11974241701863808564e0) * t8405;
    let t42384 = F::new(0.17961362552795712846e0) * t8408;
    let t42385 = F::new(0.35922725105591425692e0) * t8411;
    let t42386 = F::new(0.11974241701863808564e0) * t8414;
    let t42387 = -t8034 + t8035 + F::new(0.72732431077987577948e-1) * t7270 + t8037 + F::new(0.2909297243119503118e0) * t7280 + t8039 - F::new(0.21819729323396273384e0) * t7289 + t42383 - t42384 + t42385 - t42386;
    (t42373, t42374, t42375, t42376, t42377, t42378, t42387)
}
