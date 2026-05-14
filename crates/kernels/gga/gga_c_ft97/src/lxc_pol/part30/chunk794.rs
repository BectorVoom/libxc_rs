//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 794/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk794<F: Float>(t36186: F, t840: F, t871: F, t296: F, t36061: F, t295: F, t312: F, t36002: F, t36007: F, t1901: F, t193: F, t34108: F, t36161: F, t36165: F, t36168: F, t36172: F, t36175: F, t36179: F, t36183: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t36188 = t840 * t871 * t36186;
    let t36191 = t296 * t36061;
    let t36195 = t295 * t36002 * t312;
    let t36199 = t296 * t36007;
    let t36202 = 2.0 / 9.0 * t1901 * t36161 - 2.0 / 9.0 * t1901 * t36165 - t446 * t36168 / 3.0 - t446 * t36172 / 3.0 + 2.0 / 3.0 * t446 * t36175 - 2.0 / 3.0 * t446 * t36179 - 2.0 / 9.0 * t1901 * t36183 - t34108 + t446 * t36188 / 3.0 + 2.0 / 3.0 * t446 * t36191 + t89 * t193 * t36195 / 3.0 - t446 * t36199 / 3.0;
    (t36188, t36191, t36195, t36199, t36202)
}
