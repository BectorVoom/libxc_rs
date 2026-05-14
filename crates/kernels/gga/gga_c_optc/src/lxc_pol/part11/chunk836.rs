//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 836/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk836<F: Float>(t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t16743: F, t16747: F, t16750: F, t16756: F, t16759: F, t16763: F, t16766: F, t16800: F, t837: F) -> (F, F) {
    let t16815 = 0.30192500000000000001e0 * t13703 + 0.258925e1 * t16743 - 0.301925e0 * t16650 - 0.49671e0 * t16747 + 0.16557e0 * t16750 + 0.12077e1 * t16634 - 0.181155e1 * t16642 - 0.33547222222222222222e0 * t16630 - 0.36793333333333333333e-1 * t16756 - 0.82785e-1 * t16759 + 0.181155e1 * t16646 + 0.49671e0 * t16763 - 0.82785e-1 * t16766 - 0.60384999999999999999e0 * t16638;
    let t16816 = t16800 + t16815;
    let t16817 = t16816 * t837;
    (t16816, t16817)
}
