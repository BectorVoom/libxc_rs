//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 735/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk735<F: Float>(t144: F, t32727: F, t32993: F, t2185: F, t616: F, t7312: F, t558: F, t574: F, t7414: F, t23470: F, t5943: F, t604: F, t7407: F, t379: F, t2210: F, t160: F, t7339: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33017 = t144 * t32727;
    let t33020 = t144 * t32993;
    let t33024 = t2185 * t616 * t7312;
    let t33028 = t574 * t7414 * t558;
    let t33031 = t23470 * t5943;
    let t33034 = t604 * t7407;
    let t33035 = t33034 * t379;
    let t33036 = t2210 * t33035;
    let t33039 = t160 * t7339;
    (t33017, t33020, t33024, t33028, t33031, t33034, t33035, t33036, t33039)
}
