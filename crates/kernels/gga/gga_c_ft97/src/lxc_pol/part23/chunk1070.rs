//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1070/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1070<F: Float>(t51: F, t6041: F, t3771: F, t703: F, t8715: F, t3722: F, t9524: F, t807: F, t265: F, t42163: F, t42123: F, t10: F, t16: F, t683: F) -> (F, F, F, F, F, F, F, F) {
    let t52593 = t6041 * t51;
    let t52594 = t3771 * t52593;
    let t52679 = t8715 * t703;
    let t52861 = t9524 * t3722;
    let t52888 = t807 * t3722;
    let t53504 = t42163 * t265;
    let t53662 = t42123 * t265;
    let t53797 = t10 * t16 * t683;
    (t52593, t52594, t52679, t52861, t52888, t53504, t53662, t53797)
}
