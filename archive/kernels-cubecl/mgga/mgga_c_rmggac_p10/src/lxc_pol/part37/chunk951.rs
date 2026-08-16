//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 951/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk951<F: Float>(t77217: F, t16503: F, t35039: F, t699: F, t9169: F, t30221: F, t3194: F, t74803: F, t74807: F, t74809: F, t74813: F, t74817: F) -> (F, F, F, F, F, F, F, F) {
    let t77218 = F::cast_from(0.53205749866622299248e-5_f64) * t77217;
    let t77221 = t16503 * t35039 * t699 * t9169;
    let t77222 = F::cast_from(0.42564599893297839398e-5_f64) * t77221;
    let t77224 = F::cast_from(0.39914139006212695214e-1_f64) * t30221 * t3194;
    let t77225 = F::cast_from(0.2727466165424534173e-1_f64) * t74803;
    let t77228 = F::cast_from(0.2727466165424534173e-1_f64) * t74807;
    let t77229 = F::cast_from(0.13637330827122670865e-1_f64) * t74809;
    let t77230 = F::cast_from(0.13637330827122670865e-1_f64) * t74813;
    let t77231 = F::cast_from(0.13637330827122670865e-1_f64) * t74817;
    (t77218, t77222, t77224, t77225, t77228, t77229, t77230, t77231)
}
