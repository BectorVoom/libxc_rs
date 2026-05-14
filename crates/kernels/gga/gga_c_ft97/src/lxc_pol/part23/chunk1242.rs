//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1242/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1242<F: Float>(t18: F, t2354: F, t27468: F, t3281: F, t1424: F, t18361: F, t193: F, t6109: F, t743: F, t108142: F, t3886: F, t446: F, t122007: F, t9744: F, t4934: F, t6061: F) -> (F, F, F, F, F, F) {
    let t123876 = t3281 * t2354 * t27468 * t18;
    let t123881 = t6109 * t193 * t743 * t1424 * t18361;
    let t123883 = t108142 * t3886;
    let t123885 = t446 * t2354 * t123883;
    let t123888 = t446 * t9744 * t122007;
    let t123890 = t6061 * t4934;
    (t123876, t123881, t123883, t123885, t123888, t123890)
}
