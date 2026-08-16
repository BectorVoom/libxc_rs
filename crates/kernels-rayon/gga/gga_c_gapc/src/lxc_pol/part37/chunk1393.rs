//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1393/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1393(t34560: f64, t34563: f64, t34565: f64, t34570: f64, t34573: f64, t34576: f64, t34582: f64, t34585: f64, t34587: f64, t34589: f64, t34591: f64, t34593: f64, t34595: f64, t34597: f64, t34599: f64, t34602: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36985 = 0.10567613244746075633e-6_f64 * t34560;
    let t36986 = 0.1167337499678099199e-7_f64 * t34563;
    let t36987 = 0.13259557375557346398e-6_f64 * t34565;
    let t36989 = 0.1374296967252737644e-5_f64 * t34570;
    let t36990 = 0.67530371184977617164e-6_f64 * t34573;
    let t36991 = 0.90040494913303489552e-7_f64 * t34576;
    let t37006 = 0.57213231142828258987e-5_f64 * t34582 + 0.20220636637604418766e-5_f64 * t34585 + 0.25635144259410869702e-5_f64 * t34587 - 0.67530371184977617164e-6_f64 * t34589 + 0.39778672126672039194e-6_f64 * t34591 - 0.21587406280859666178e-5_f64 * t34593 + 0.18550690221634253912e-3_f64 * t34595 + 0.15458908518028544927e-5_f64 * t34597 - 0.2748593934505475288e-5_f64 * t34599 - 0.84412963981222021456e-7_f64 * t34602;
    (t36985, t36986, t36987, t36989, t36990, t36991, t37006)
}
