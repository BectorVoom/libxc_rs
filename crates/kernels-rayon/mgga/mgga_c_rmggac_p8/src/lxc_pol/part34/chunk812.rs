//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 812/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk812(t1986: f64, t675: f64, t8958: f64, t13862: f64, t1654: f64, t3133: f64, t14011: f64, t1603: f64, t3120: f64, t14150: f64, t290: f64, t39116: f64, t70127: f64) -> (f64, f64, f64, f64) {
    let t74584 = t675 * t1986 * t8958;
    let t74587 = t3133 * t13862 * t1654;
    let t74590 = t3120 * t14011 * t1603;
    let t74594 = t70127 * t39116 * t290 * t14150;
    (t74584, t74587, t74590, t74594)
}
