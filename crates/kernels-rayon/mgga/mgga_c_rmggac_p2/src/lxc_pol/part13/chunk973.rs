//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 973/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk973(t36110: f64, t41304: f64, t41307: f64, t7603: f64, t36103: f64, t41310: f64, t41313: f64, t25607: f64, t27: f64, t41316: f64, t3851: f64, t39688: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41321 = t36110 * t41304;
    let t41323 = t7603 * t41307;
    let t41325 = t36103 * t41310;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41330 = t41329 * t41316;
    let t41332 = t3851 * t39688;
    (t41321, t41323, t41325, t41327, t41330, t41332)
}
