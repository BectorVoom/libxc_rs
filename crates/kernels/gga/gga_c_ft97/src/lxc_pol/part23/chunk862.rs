//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 862/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk862<F: Float>(t27574: F, t6820: F, t6815: F, t24330: F, t6043: F, t6824: F, t6808: F, t6809: F, t1109: F, t6022: F) -> (F, F, F, F, F) {
    let t27575 = t27574 * t6820;
    let t27576 = t6815 * t27575;
    let t27579 = t6043 * t24330 * t6824;
    let t27582 = t6808 * t24330 * t6809;
    let t27584 = t6022 * t1109;
    (t27575, t27576, t27579, t27582, t27584)
}
