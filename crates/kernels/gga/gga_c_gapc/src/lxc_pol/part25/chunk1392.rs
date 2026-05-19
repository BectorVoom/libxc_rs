//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1392/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1392<F: Float>(t34560: F, t34563: F, t34565: F, t34570: F, t34573: F, t34576: F, t34582: F, t34585: F, t34587: F, t34589: F, t34591: F, t34593: F, t34595: F, t34597: F, t34599: F, t34602: F) -> (F, F, F, F, F, F, F) {
    let t36985 = F::cast_from(0.10567613244746075633e-6_f64) * t34560;
    let t36986 = F::cast_from(0.1167337499678099199e-7_f64) * t34563;
    let t36987 = F::cast_from(0.13259557375557346398e-6_f64) * t34565;
    let t36989 = F::cast_from(0.1374296967252737644e-5_f64) * t34570;
    let t36990 = F::cast_from(0.67530371184977617164e-6_f64) * t34573;
    let t36991 = F::cast_from(0.90040494913303489552e-7_f64) * t34576;
    let t37006 = F::cast_from(0.57213231142828258987e-5_f64) * t34582 + F::cast_from(0.20220636637604418766e-5_f64) * t34585 + F::cast_from(0.25635144259410869702e-5_f64) * t34587 - F::cast_from(0.67530371184977617164e-6_f64) * t34589 + F::cast_from(0.39778672126672039194e-6_f64) * t34591 - F::cast_from(0.21587406280859666178e-5_f64) * t34593 + F::cast_from(0.18550690221634253912e-3_f64) * t34595 + F::cast_from(0.15458908518028544927e-5_f64) * t34597 - F::cast_from(0.2748593934505475288e-5_f64) * t34599 - F::cast_from(0.84412963981222021456e-7_f64) * t34602;
    (t36985, t36986, t36987, t36989, t36990, t36991, t37006)
}
