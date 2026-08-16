//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1019/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1019(t5226: f64, t649: f64, t36107: f64, t1632: f64, t2084: f64, t7599: f64, t5223: f64, t41130: f64, t41301: f64, t8750: f64, t36110: f64, t41304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41310 = t649 * t5226;
    let t41311 = t36107 * t41310;
    let t41313 = t2084 * t1632;
    let t41314 = t7599 * t41313;
    let t41315 = 0.72732431077987577946e-1_f64 * t41314;
    let t41316 = t649 * t5223;
    let t41317 = t41130 * t41316;
    let t41319 = t8750 * t41301;
    let t41320 = 0.2419210303588817044e-2_f64 * t41319;
    let t41321 = t36110 * t41304;
    (t41310, t41311, t41313, t41315, t41316, t41317, t41320, t41321)
}
