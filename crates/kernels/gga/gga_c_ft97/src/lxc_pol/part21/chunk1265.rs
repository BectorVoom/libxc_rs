//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1265/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1265<F: Float>(t16950: F, t23909: F, t5899: F, t95340: F, t23657: F, t23671: F, t27091: F, t925: F, t17066: F, t40830: F, t5900: F, t105412: F, t105414: F, t119598: F, t119602: F, t119606: F, t119610: F, t119614: F, t119618: F, t119623: F) -> (F, F, F, F, F) {
    let t119625 = t23909 * t16950;
    let t119627 = t5899 * t95340 * t119625;
    let t119631 = t23657 * t23671 * t27091 * t925;
    let t119635 = t5899 * t40830 * t5900 * t17066;
    let t119638 = -t119598 / 3.0 + t119602 / 8.0 + 3.0 / 2.0 * t119606 - t119610 / 3.0 - t119614 / 3.0 + t119618 / 9.0 - t119623 / 12.0 - 2.0 / 9.0 * t119627 - t119631 / 6.0 + 12.0 * t119635 - t105412 - 2.0 / 9.0 * t105414;
    (t119625, t119627, t119631, t119635, t119638)
}
