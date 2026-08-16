//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1104/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1104(t2466: f64, t28295: f64, t2868: f64, t36402: f64, t40654: f64, t40679: f64, t40681: f64, t43680: f64, t47196: f64, t47202: f64, t47207: f64, t47213: f64, t47215: f64, t47219: f64, t47223: f64, t47225: f64, t47229: f64, t9437: f64, t9620: f64) -> f64 {
    let t48946 = -0.79453919800822633544e-4_f64 * t40654 - 0.10215503974391481456e-3_f64 * t47196 + 0.15323255961587222184e-3_f64 * t47202 - 0.20431007948782962912e-3_f64 * t47207 + 0.20001418546446583936e0_f64 * t36402 - 0.11974241701863808564e0_f64 * t2868 * t9437 + 0.11974241701863808564e0_f64 * t28295 * t2466 - 0.16552899958504715322e-3_f64 * t40679 - 0.13242319966803772257e-3_f64 * t40681 + 2.0_f64 * t43680 - 0.5987120850931904282e-1_f64 * t47213 + 0.5959043985061697516e-4_f64 * t47215 + 0.23948483403727617128e0_f64 * t2868 * t9620 + 0.3405167991463827152e-4_f64 * t47219 - 0.1702583995731913576e-4_f64 * t47223 + 0.47885174879960069325e-4_f64 * t47225 - 0.5107751987195740728e-4_f64 * t47229;
    t48946
}
