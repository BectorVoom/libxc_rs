//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 636/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk636<F: Float>(t28255: F, t729: F, t762: F, t242: F, t27984: F, t2574: F, t265: F, t27878: F, t6837: F, t766: F, t2469: F, t6861: F) -> (F, F, F, F, F, F) {
    let t28257 = t729 * t762 * t28255;
    let t28260 = t242 * t27984;
    let t28264 = t2574 * t265 * t27878;
    let t28267 = t6837 * t766;
    let t28269 = t729 * t762 * t28267;
    let t28273 = t729 * t2469 * t6861;
    (t28257, t28260, t28264, t28267, t28269, t28273)
}
