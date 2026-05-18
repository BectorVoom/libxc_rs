//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 887/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk887<F: Float>(t2185: F, t35050: F, t605: F, t167: F, t34822: F, t3578: F, t574: F, t7357: F, t33039: F, t925: F, t2221: F, t27015: F, t6708: F) -> (F, F, F, F, F, F) {
    let t35052 = t2185 * t605 * t35050;
    let t35056 = t2185 * t167 * t34822;
    let t35060 = t574 * t3578 * t7357;
    let t35063 = t33039 * t925;
    let t35064 = t2221 * t35063;
    let t35067 = t27015 * t6708;
    (t35052, t35056, t35060, t35063, t35064, t35067)
}
