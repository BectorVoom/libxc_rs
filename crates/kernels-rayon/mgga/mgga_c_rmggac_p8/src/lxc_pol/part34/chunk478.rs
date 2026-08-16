//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 478/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk478(t13848: f64, t13850: f64, t1977: f64, t2190: f64, t3148: f64, t3151: f64, t2191: f64, t3154: f64, t1986: f64, t2125: f64, t675: f64, t1004: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13851 = t1977 * t13848 * t13850;
    let t13854 = t2190 * t3148 * t3151;
    let t13856 = t2191 * t3154;
    let t13858 = t1986 * t2125;
    let t13859 = t675 * t13858;
    let t13861 = t7 * t1004;
    (t13851, t13854, t13856, t13858, t13859, t13861)
}
