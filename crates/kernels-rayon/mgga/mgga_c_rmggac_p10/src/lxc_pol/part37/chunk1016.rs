//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1016/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1016(t70176: f64, t70208: f64, t1356: f64, t78022: f64, t77980: f64, t2392: f64, t739: f64, t8264: f64, t2211: f64, t8924: f64, t76027: f64, t76029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78406 = 0.638468998399467591e-4_f64 * t70176;
    let t78409 = 0.79808624799933448875e-4_f64 * t70208;
    let t78423 = 0.39914139006212695214e-1_f64 * t1356 * t78022;
    let t78427 = 0.39914139006212695214e-1_f64 * t1356 * t77980;
    let t78430 = t739 * t8264 * t2392;
    let t78431 = 0.2993560425465952141e-1_f64 * t78430;
    let t78433 = t739 * t2211 * t8924;
    let t78434 = 0.2993560425465952141e-1_f64 * t78433;
    let t78436 = 0.38430329123504567781e-4_f64 * t76027;
    let t78438 = 0.1276937996798935182e-4_f64 * t76029;
    (t78406, t78409, t78423, t78427, t78431, t78434, t78436, t78438)
}
