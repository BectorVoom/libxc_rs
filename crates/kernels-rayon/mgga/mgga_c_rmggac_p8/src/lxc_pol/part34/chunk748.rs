//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 748/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk748(t71835: f64, t739: f64, t14531: f64, t275: f64, t69108: f64, t69114: f64, t14512: f64, t7269: f64, t14509: f64, t7279: f64, t2228: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71836 = t739 * t71835;
    let t71850 = t275 * t14531;
    let t71852 = 0.10492326631435615411e0_f64 * t69108;
    let t71854 = 0.66671395154821946452e-1_f64 * t69114;
    let t71863 = t14512 * t7269;
    let t71871 = t14509 * t7279;
    let t71876 = t797 * t2228;
    (t71836, t71850, t71852, t71854, t71863, t71871, t71876)
}
