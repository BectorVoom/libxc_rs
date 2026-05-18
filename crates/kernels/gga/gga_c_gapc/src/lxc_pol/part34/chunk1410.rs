//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1410/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1410<F: Float>(t34942: F, t34946: F, t34951: F, t34956: F, t34958: F, t34960: F, t34965: F, t34949: F, t34954: F, t34962: F, t37134: F, t34973: F) -> (F, F) {
    let t37135 = F::new(0.40518222710986570299e-5) * t34942;
    let t37136 = F::new(0.17679409834076461864e-7) * t34946;
    let t37138 = F::new(0.50603841145833333336e-5) * t34951;
    let t37140 = F::new(0.26519114751114692796e-6) * t34956;
    let t37141 = F::new(0.26519114751114692796e-6) * t34958;
    let t37142 = F::new(0.13259557375557346398e-6) * t34960;
    let t37144 = F::new(0.45020247456651744776e-7) * t34965;
    let t37145 = t37134 - t37135 + t37136 - F::new(0.18115908419564701085e-6) * t34949 - t37138 - F::new(0.98380106748709416168e-8) * t34954 - t37140 - t37141 - t37142 - F::new(0.18115908419564701085e-6) * t34962 - t37144;
    let t37149 = F::new(0.12141398358188788626e-5) * t34973;
    (t37145, t37149)
}
