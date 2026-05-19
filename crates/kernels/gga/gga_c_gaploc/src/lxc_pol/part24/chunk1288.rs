//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1288/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1288<F: Float>(t2615: F, t28406: F, t28408: F, t28410: F, t28415: F, t28419: F, t28421: F, t28423: F, t28425: F, t28427: F, t28437: F, t28441: F, t326: F, t32796: F, t33151: F, t33154: F, t33158: F, t33164: F) -> F {
    let t33165 = t33151 - t33154 - t28406 - t28408 + t28410 - t28415 + t33158 + F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t32796 - t33164 - t28419 - t28421 - t28423 + t28425 + t28427 + t28437 - t28441;
    t33165
}
