//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 313/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk313<F: Float>(t292: F, t2735: F, t817: F, t2689: F, t2691: F, t2693: F, t2720: F, t2727: F, t285: F, t800: F) -> (F, F) {
    let t293 = 0.1e-59 < t292;
    let t2736 = t817 * t2735;
    let t2739 = piecewise3(t293, -4.0 * t2691 * t2693 + 2.0 * t2720 * t800 + 2.0 * t2727 * t285 - t2736 * t285 + 2.0 * t2689, 0.0);
    (t2736, t2739)
}
