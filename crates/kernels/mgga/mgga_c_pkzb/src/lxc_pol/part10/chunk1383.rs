//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1383/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1383<F: Float>(t18427: F, t18430: F, t18448: F, t18765: F, t18766: F, t27256: F, t27262: F, t27289: F, t27292: F, t27295: F, t27305: F, t18445: F, t18451: F, t22230: F, t22233: F, t22236: F, t27308: F, t27311: F, t27318: F, t27320: F, t27323: F, t27325: F, t27327: F) -> (F, F) {
    let t27723 = 0.62517e0 * t27256 + t18765 - 0.32136222222222222222e1 * t18427 + 0.68863333333333333333e0 * t18430 + t18766 + 0.34731666666666666666e0 * t18448 - 0.103295e1 * t27262 + 0.1549425e1 * t27289 + 0.34731666666666666667e0 * t27292 + 0.68863333333333333333e0 * t27295 + 0.3529725e1 * t27305;
    let t27736 = 0.6311625e0 * t27308 + 0.6311625e0 * t27311 - 0.18523555555555555555e1 * t18445 + 0.34731666666666666666e0 * t18451 - 0.32136222222222222223e1 * t22230 + 0.27545333333333333334e1 * t22233 - 0.103295e1 * t22236 + 0.264729375e1 * t27318 - 0.3529725e1 * t27320 - 0.3529725e1 * t27323 - 0.17648625e1 * t27325 - 0.157790625e0 * t27327;
    (t27723, t27736)
}
