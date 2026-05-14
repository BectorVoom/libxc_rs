//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1198/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1198<F: Float>(t34958: F, t34960: F, t34965: F, t34949: F, t34954: F, t34962: F, t37134: F, t37135: F, t37136: F, t37138: F, t37140: F, t34973: F, t34975: F, t34977: F, t34982: F, t34984: F) -> (F, F, F, F, F, F) {
    let t37141 = 0.26519114751114692796e-6 * t34958;
    let t37142 = 0.13259557375557346398e-6 * t34960;
    let t37144 = 0.45020247456651744776e-7 * t34965;
    let t37145 = t37134 - t37135 + t37136 - 0.18115908419564701085e-6 * t34949 - t37138 - 0.98380106748709416168e-8 * t34954 - t37140 - t37141 - t37142 - 0.18115908419564701085e-6 * t34962 - t37144;
    let t37149 = 0.12141398358188788626e-5 * t34973;
    let t37150 = 0.21103240995305505364e-7 * t34975;
    let t37151 = 0.42206481990611010728e-7 * t34977;
    let t37153 = 0.21103240995305505364e-7 * t34982;
    let t37154 = 0.90040494913303489553e-6 * t34984;
    (t37145, t37149, t37150, t37151, t37153, t37154)
}
