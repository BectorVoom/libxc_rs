//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 522/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk522<F: Float>(t359: F, t7745: F, t356: F, t89: F, t23: F, t7241: F, t1588: F, t432: F) -> (F, F, F, F) {
    let t7746 = t359 * t7745;
    let t7748 = t89 * t356 * t7746;
    let t7750 = t23 * t7241;
    let t7751 = t1588 * t432;
    (t7746, t7748, t7750, t7751)
}
