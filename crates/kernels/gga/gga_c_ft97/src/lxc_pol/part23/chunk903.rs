//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 903/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk903<F: Float>(t28023: F, t766: F, t24232: F, t3875: F, t24231: F, t1425: F, t683: F, t2360: F, t263: F, t3886: F, t2404: F) -> (F, F, F, F, F, F, F, F) {
    let t28024 = t28023 * t766;
    let t28026 = t24232 * t3875;
    let t28027 = t24231 * t28026;
    let t28030 = t683 * t1425;
    let t28031 = t263 * t2360;
    let t28032 = t28031 * t3886;
    let t28033 = t28030 * t28032;
    let t28036 = t2404 * t1425;
    (t28024, t28026, t28027, t28030, t28031, t28032, t28033, t28036)
}
