//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 979/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk979(t74807: f64, t74809: f64, t74813: f64, t74817: f64, t15598: f64, t333: f64, t74835: f64, t74839: f64, t74858: f64, t74861: f64, t74864: f64, t74824: f64, t74830: f64, t74831: f64, t74842: f64, t74846: f64, t74850: f64, t74856: f64, t884: f64) -> (f64, f64) {
    let t77228 = 0.2727466165424534173e-1_f64 * t74807;
    let t77229 = 0.13637330827122670865e-1_f64 * t74809;
    let t77230 = 0.13637330827122670865e-1_f64 * t74813;
    let t77231 = 0.13637330827122670865e-1_f64 * t74817;
    let t77233 = t15598 * t333;
    let t77236 = 0.69805943825008456614e-4_f64 * t74835;
    let t77237 = 0.11634323970834742769e-3_f64 * t74839;
    let t77242 = 0.1276937996798935182e-4_f64 * t74858;
    let t77243 = 0.1276937996798935182e-4_f64 * t74861;
    let t77244 = 0.638468998399467591e-4_f64 * t74864;
    let t77245 = t77228 + t77229 + t77230 + t77231 - t74824 + t74830 - 0.58171619854173713846e-5_f64 * t74831 + 0.59871208509319042821e-1_f64 * t884 * t77233 - t77236 + t77237 + 0.17519306092901367187e-5_f64 * t74842 + 0.35038612185802734376e-6_f64 * t74846 - 0.35038612185802734376e-6_f64 * t74850 + 0.8759653046450683594e-6_f64 * t74856 + t77242 - t77243 + t77244;
    (t77233, t77245)
}
