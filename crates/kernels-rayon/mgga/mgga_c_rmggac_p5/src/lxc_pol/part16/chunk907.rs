//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 907/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk907(t511: f64, t6349: f64, t2136: f64, t2186: f64, t9938: f64, t2191: f64, t9731: f64, t1986: f64, t6602: f64, t675: f64, t1392: f64, t1979: f64, t1982: f64, t201: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t45086 = t6349 * t511;
    let t45087 = t45086 * t2136;
    let t45089 = t2186 * t9938;
    let t45091 = t2191 * t9731;
    let t45094 = t675 * t1986 * t6602;
    let t45099 = t1392 * t597 * t201 * t1979 * t1982;
    (t45087, t45089, t45091, t45094, t45099)
}
