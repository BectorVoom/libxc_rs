//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 421/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk421<F: Float>(t2999: F, t355: F, t18: F, t359: F, t89: F, t375: F, t943: F) -> (F, F, F, F) {
    let t3000 = t2999 * t355;
    let t3001 = t359 * t18;
    let t3003 = t89 * t3000 * t3001;
    let t3006 = t89 * t375 * t943;
    (t3000, t3001, t3003, t3006)
}
