//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 851/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk851(t2046: f64, t3047: f64, t8858: f64, t8862: f64, t15220: f64, t2186: f64, t2051: f64, t577: f64, t68417: f64, t68406: f64, t15166: f64, t36639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75238 = t2046 * t3047 * t8858;
    let t75241 = t2046 * t3047 * t8862;
    let t75247 = t2186 * t15220;
    let t75248 = 0.19863479950205658386e-4_f64 * t75247;
    let t75249 = t577 * t2051;
    let t75250 = t68417 * t75249;
    let t75252 = t68406 * t75249;
    let t75254 = t36639 * t15166;
    (t75238, t75241, t75248, t75250, t75252, t75254)
}
