//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 944/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk944<F: Float>(t33489: F, t7963: F, t7965: F, t4210: F, t7942: F, t315: F, t5386: F, t610: F, t2132: F, t322: F, t7896: F, t8422: F) -> (F, F, F, F) {
    let t33621 = F::new(0.17347256376410398924e1) * t7963 * t33489 * t7965;
    let t33624 = F::new(0.17347256376410398924e1) * t7942 * t33489 * t4210;
    let t33627 = F::new(0.26341796731742046394e1) * t315 * t610 * t5386;
    let t33635 = t7896 * t2132 * t8422 * t322;
    (t33621, t33624, t33627, t33635)
}
