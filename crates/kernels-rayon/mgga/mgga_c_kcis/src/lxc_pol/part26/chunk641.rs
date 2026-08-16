//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 641/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk641(t1495: f64, t7202: f64, t1395: f64, t1464: f64, t2002: f64, t5748: f64, t3752: f64, t3755: f64, t6281: f64, t1889: f64, t1897: f64, t3761: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7203 = t1495 * t7202;
    let t7204 = t1395 * t7203;
    let t7205 = t1464 * t7204;
    let t7207 = t5748 * t2002;
    let t7208 = t1464 * t7207;
    let t7214 = t3752 * t3755 * t6281;
    let t7218 = t3761 * t1889 * t1897;
    (t7203, t7204, t7205, t7207, t7208, t7214, t7218)
}
