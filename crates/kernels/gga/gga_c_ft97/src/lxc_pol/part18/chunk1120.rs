//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1120/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1120<F: Float>(t5838: F, t92834: F, t92837: F, t92831: F, t23878: F, t5608: F, t22632: F, t23758: F, t5829: F, t5830: F, t92433: F, t23754: F, t5813: F, t23721: F, t94607: F, t1995: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94620 = t5838 * t92834;
    let t94622 = t5838 * t92837;
    let t94626 = t5838 * t92831;
    let t94640 = t23878 * t5608;
    let t94666 = t5829 * t22632 * t23758;
    let t94686 = t5829 * t92433 * t5830;
    let t94689 = t5813 * t22632 * t23754;
    let t94697 = t23721 * t94607;
    let t94700 = t1995 * t94607;
    (t94620, t94622, t94626, t94640, t94666, t94686, t94689, t94697, t94700)
}
