//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 927/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk927<F: Float>(t29502: F, t73: F, t4474: F, t72: F, t4491: F, t373: F, t4466: F, t35: F) -> (F, F, F, F, F) {
    let t29503 = t73 * t29502;
    let t29506 = t72 * t4474;
    let t29510 = t72 * t4491;
    let t29514 = t373 * t4466;
    let t29515 = t29514 * t35;
    (t29503, t29506, t29510, t29514, t29515)
}
