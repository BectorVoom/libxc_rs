//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 465/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk465<F: Float>(t2131: F, t2226: F, t322: F, t633: F, t2132: F) -> (F, F, F) {
    let t2228 = F::cast_from(0.8673628188205199462e0_f64) * t2131 * t2226;
    let t2229 = t633 * t322;
    let t2230 = t2132 * t2229;
    (t2228, t2229, t2230)
}
