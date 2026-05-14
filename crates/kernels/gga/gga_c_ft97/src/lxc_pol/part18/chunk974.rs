//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 974/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk974<F: Float>(t23076: F, t23081: F, t25999: F, t26004: F, t26009: F, t26014: F, t26019: F, t26022: F, t26025: F, t26029: F, t26033: F, t26036: F, t26077: F, t26089: F, t26102: F) -> (F,) {
    let t26111 = t25999 + t26004 + t26009 / 4.0 + t26014 / 4.0 + t26019 / 4.0 - t26022 / 3.0 - t26025 / 12.0 - t26029 - t26033 / 2.0 + t26036 / 6.0 - t23076 + t23081 / 3.0;
    let t26113 = t26077 + t26089 + t26102 + t26111;
    (t26113,)
}
