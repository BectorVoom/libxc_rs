//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 513/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk513<F: Float>(t1255: F, t377: F, t457: F, t980: F, t313: F, t111: F, t150: F, t322: F, t864: F) -> (F, F, F, F, F, F, F) {
    let t3029 = t377 * t1255;
    let t3031 = t980 * t457;
    let t3033 = t313 * t313;
    let t3034 = F::cast_from(1.0_f64) / t3033;
    let t3035 = t111 * t3034;
    let t3036 = t3035 * t150;
    let t3037 = t864 * t322;
    (t3029, t3031, t3033, t3034, t3035, t3036, t3037)
}
