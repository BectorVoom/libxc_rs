//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 699/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk699(t9982: f64, t236: f64, t6172: f64, t1971: f64, t1970: f64, t615: f64, t618: f64) -> (f64, f64, f64, f64) {
    let t9983 = 0.85129199786595678796e-5_f64 * t9982;
    let t9984 = t236 * t6172;
    let t9985 = t1971 * t9984;
    let t9986 = t1970 * t9985;
    let t9987 = 0.42564599893297839398e-5_f64 * t9986;
    let t9988 = t618 * t615;
    (t9983, t9985, t9987, t9988)
}
