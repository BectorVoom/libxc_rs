//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 951/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk951(t77217: f64, t16503: f64, t35039: f64, t699: f64, t9169: f64, t30221: f64, t3194: f64, t74803: f64, t74807: f64, t74809: f64, t74813: f64, t74817: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77218 = 0.53205749866622299248e-5_f64 * t77217;
    let t77221 = t16503 * t35039 * t699 * t9169;
    let t77222 = 0.42564599893297839398e-5_f64 * t77221;
    let t77224 = 0.39914139006212695214e-1_f64 * t30221 * t3194;
    let t77225 = 0.2727466165424534173e-1_f64 * t74803;
    let t77228 = 0.2727466165424534173e-1_f64 * t74807;
    let t77229 = 0.13637330827122670865e-1_f64 * t74809;
    let t77230 = 0.13637330827122670865e-1_f64 * t74813;
    let t77231 = 0.13637330827122670865e-1_f64 * t74817;
    (t77218, t77222, t77224, t77225, t77228, t77229, t77230, t77231)
}
