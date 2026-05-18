//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1027/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1027<F: Float>(t1564: F, t446: F, t85783: F, t86058: F, t86082: F, t1866: F, t85491: F, t85465: F, t7793: F, t86161: F, t37305: F, t86068: F) -> (F, F, F, F, F, F, F) {
    let t86199 = t446 * t1564 * t85783;
    let t86202 = t446 * t1564 * t86058;
    let t86205 = t446 * t1564 * t86082;
    let t86208 = t446 * t1866 * t85491;
    let t86211 = t446 * t1866 * t85465;
    let t86214 = t446 * t7793 * t86161;
    let t86217 = t446 * t37305 * t86068;
    (t86199, t86202, t86205, t86208, t86211, t86214, t86217)
}
