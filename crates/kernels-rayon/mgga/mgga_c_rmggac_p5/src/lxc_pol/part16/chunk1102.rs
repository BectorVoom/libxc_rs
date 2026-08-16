//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1102/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1102(t1743: f64, t2228: f64, t1614: f64, t2471: f64, t10350: f64, t2265: f64, t2604: f64, t37860: f64, t37866: f64, t37872: f64, t43481: f64, t47110: f64, t47112: f64, t47114: f64, t47119: f64, t47133: f64, t47138: f64, t47142: f64, t47146: f64, t47152: f64, t5879: f64, t884: f64) -> (f64, f64, f64) {
    let t48894 = t2228 * t1743;
    let t48897 = t2471 * t1614;
    let t48901 = 0.11974241701863808564e0_f64 * t47110 - 0.1702583995731913576e-4_f64 * t47112 - 0.1702583995731913576e-4_f64 * t47114 - 0.1702583995731913576e-4_f64 * t47119 + t37860 - 0.11974241701863808564e0_f64 * t2604 * t10350 + t43481 - 0.5959043985061697516e-4_f64 * t47133 - 0.2363e1_f64 * t5879 * t2265 + 0.19863479950205658386e-4_f64 * t47138 - 0.2363e1_f64 * t37866 - 0.15323255961587222184e-3_f64 * t47142 - 0.15323255961587222184e-3_f64 * t47146 + 0.59871208509319042821e-1_f64 * t884 * t48894 + 0.11974241701863808564e0_f64 * t884 * t48897 - t37872 - 0.10215503974391481456e-3_f64 * t47152;
    (t48894, t48897, t48901)
}
