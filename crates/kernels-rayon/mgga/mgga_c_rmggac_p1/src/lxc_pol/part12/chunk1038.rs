//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1038/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1038(t34944: f64, t40888: f64, t22: f64, t235: f64, t26115: f64, t40902: f64, t40921: f64, t8630: f64, t36978: f64, t40894: f64, t40898: f64, t7198: f64) -> (f64, f64, f64, f64, f64) {
    let t41631 = t34944 * t40888;
    let t41634 = t235 * t26115 * t22;
    let t41635 = t41634 * t40902;
    let t41637 = t8630 * t40921;
    let t41639 = t36978 * t40894;
    let t41641 = t7198 * t40898;
    (t41631, t41635, t41637, t41639, t41641)
}
