//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 861/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk861<F: Float>(t24311: F, t27561: F, t24389: F, t6: F, t17836: F, t24330: F, t6832: F, t6055: F, t172: F, t6818: F) -> (F, F, F, F, F, F) {
    let t27562 = t24311 * t27561;
    let t27565 = t24389 * t6;
    let t27566 = t17836 * t27565;
    let t27569 = t24330 * t6832;
    let t27570 = t6055 * t27569;
    let t27574 = t6818 * t172;
    (t27562, t27565, t27566, t27569, t27570, t27574)
}
