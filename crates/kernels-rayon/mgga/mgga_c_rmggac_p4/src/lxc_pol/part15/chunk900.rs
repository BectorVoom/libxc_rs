//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 900/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk900(t1970: f64, t1971: f64, t333: f64, t511: f64, t6172: f64, t6349: f64, t2136: f64, t2186: f64, t9938: f64, t2191: f64, t9731: f64, t1986: f64, t6602: f64, t675: f64) -> (f64, f64, f64, f64, f64) {
    let t45080 = t1970 * t1971 * t511 * t6172 * t333;
    let t45086 = t6349 * t511;
    let t45087 = t45086 * t2136;
    let t45089 = t2186 * t9938;
    let t45091 = t2191 * t9731;
    let t45094 = t675 * t1986 * t6602;
    (t45080, t45087, t45089, t45091, t45094)
}
