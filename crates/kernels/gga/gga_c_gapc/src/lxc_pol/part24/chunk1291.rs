//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1291/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1291<F: Float>(t35928: F, t35930: F, t35932: F, t35934: F, t35938: F, t35940: F, t35943: F, t35945: F, t35948: F, t35954: F, t35956: F, t35959: F, t35962: F) -> F {
    let t37584 = -F::new(0.64586396578113893434e-4) * t35928 - F::new(0.28452260327530379456e-3) * t35930 + F::new(0.64586396578113893434e-4) * t35932 + F::new(0.77948343448359322925e-4) * t35934 - F::new(0.87274686751864770716e-7) * t35938 - F::new(0.16703216453219854913e-4) * t35940 - F::new(0.16703216453219854913e-4) * t35943 + F::new(0.45596571037708941436e-6) * t35945 + F::new(0.2188635409810029189e-4) * t35948 + F::new(0.24974222161675984676e-6) * t35954 - F::new(0.2188635409810029189e-4) * t35956 + F::new(0.46971924784082831588e-4) * t35959 + F::new(0.93943849568165663176e-4) * t35962;
    t37584
}
