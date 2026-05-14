//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 726/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk726<F: Float>(t7313: F, t7326: F, t7336: F, t7346: F, t7355: F, t7387: F, t7492: F, t7559: F, t7562: F, t7767: F, t8201: F, t7901: F, t34687: F, t34704: F, t34706: F, t34710: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37086 = 0.18292589874945016987e-2 * t7313;
    let t37089 = 0.205201155180140685e-5 * t7326;
    let t37096 = 0.91462949374725084936e-3 * t7336;
    let t37099 = 0.91462949374725084936e-3 * t7346;
    let t37100 = 0.26021382394247697185e-3 * t7355;
    let t37108 = 0.91462949374725084936e-3 * t7387;
    let t37134 = 0.18292589874945016987e-2 * t7492;
    let t37147 = 0.26021382394247697185e-3 * t7559;
    let t37148 = 0.20001418546446583935e0 * t7562;
    let t37179 = 0.18292589874945016987e-2 * t7767;
    let t37183 = 3.0 * t8201;
    let t37186 = 0.87811105813667929468e0 * t7901;
    let t37200 = 0.205201155180140685e-5 * t34687;
    let t37201 = 0.18292589874945016987e-2 * t34704;
    let t37202 = 0.91462949374725084936e-3 * t34706;
    let t37203 = 0.13010691197123848592e-3 * t34710;
    (t37086, t37089, t37096, t37099, t37100, t37108, t37134, t37147, t37148, t37179, t37183, t37186, t37200, t37201, t37202, t37203)
}
