//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 817/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk817(t577: f64, t669: f64, t7933: f64, t7934: f64, t35688: f64, t70171: f64, t9081: f64, t11674: f64, t498: f64, t14236: f64, t2067: f64, t69629: f64) -> (f64, f64, f64) {
    let t74722 = t7933 * t7934 * t577 * t669;
    let t74725 = t35688 * t70171 * t9081;
    let t74727 = t11674 * t498;
    let t74730 = t14236 * t69629 * t2067 * t74727;
    (t74722, t74725, t74730)
}
