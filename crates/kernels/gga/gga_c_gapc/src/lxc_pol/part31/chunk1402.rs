//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1402/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1402<F: Float>(t34936: F, t34940: F, t34942: F, t34946: F, t34951: F, t34956: F, t34958: F, t34960: F, t34965: F, t34973: F, t34975: F, t34977: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37132 = F::cast_from(0.13506074236995523433e-5_f64) * t34936;
    let t37134 = F::cast_from(0.16009199995585360443e-6_f64) * t34940;
    let t37135 = F::cast_from(0.40518222710986570299e-5_f64) * t34942;
    let t37136 = F::cast_from(0.17679409834076461864e-7_f64) * t34946;
    let t37138 = F::cast_from(0.50603841145833333336e-5_f64) * t34951;
    let t37140 = F::cast_from(0.26519114751114692796e-6_f64) * t34956;
    let t37141 = F::cast_from(0.26519114751114692796e-6_f64) * t34958;
    let t37142 = F::cast_from(0.13259557375557346398e-6_f64) * t34960;
    let t37144 = F::cast_from(0.45020247456651744776e-7_f64) * t34965;
    let t37149 = F::cast_from(0.12141398358188788626e-5_f64) * t34973;
    let t37150 = F::cast_from(0.21103240995305505364e-7_f64) * t34975;
    let t37151 = F::cast_from(0.42206481990611010728e-7_f64) * t34977;
    (t37132, t37134, t37135, t37136, t37138, t37140, t37141, t37142, t37144, t37149, t37150, t37151)
}
