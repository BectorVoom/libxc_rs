//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 995/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk995(t75024: f64, t75033: f64, t75037: f64, t69091: f64, t69094: f64, t71315: f64, t71316: f64, t75010: f64, t75029: f64, t75040: f64, t77418: f64, t77421: f64, t77424: f64, t77425: f64, t77426: f64, t77427: f64) -> f64 {
    let t77428 = 0.638468998399467591e-4_f64 * t75024;
    let t77430 = 0.23268647941669485538e-4_f64 * t75033;
    let t77431 = 0.23268647941669485538e-4_f64 * t75037;
    let t77433 = 0.58171619854173713846e-5_f64 * t75010 - t71315 + t71316 + 0.6505345598561924296e-5_f64 * t69091 + 0.6505345598561924296e-5_f64 * t69094 + t77418 + t77421 - t77424 - t77425 - t77426 + t77427 - t77428 + 0.87596530464506835935e-6_f64 * t75029 + t77430 - t77431 + 0.17519306092901367187e-5_f64 * t75040;
    t77433
}
