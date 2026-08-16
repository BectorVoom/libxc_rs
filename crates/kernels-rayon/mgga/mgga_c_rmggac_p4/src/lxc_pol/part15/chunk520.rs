//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 520/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk520(t623: f64, t837: f64, t234: f64, t321: f64, t1598: f64, t1652: f64, t1953: f64, t68: f64, t131: f64, t1926: f64, t333: f64, t1933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6473 = t623 * t837;
    let t6477 = t234 * t321;
    let t6482 = t1598 * t1652;
    let t6491 = t68 * t1953;
    let t6492 = t6491 * t131;
    let t6495 = t1926 * t333;
    let t6501 = t1933 * t321;
    (t6473, t6477, t6482, t6491, t6492, t6495, t6501)
}
