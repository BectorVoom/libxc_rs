//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1321/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1321<F: Float>(t23652: F, t3450: F, t5899: F, t9432: F, t23649: F, t27074: F, t13021: F, t5916: F, t23667: F, t27120: F, t1647: F, t6630: F, t23657: F, t23671: F, t27091: F, t379: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t105508 = t5899 * t9432 * t23652 * t3450;
    let t105510 = t23649 * t27074;
    let t105511 = 2.0 / 27.0 * t105510;
    let t105512 = t5916 * t13021;
    let t105514 = t5899 * t23667 * t105512;
    let t105516 = t23649 * t27120;
    let t105517 = 2.0 / 9.0 * t105516;
    let t105518 = t6630 * t1647;
    let t105520 = t5899 * t23667 * t105518;
    let t105524 = t23657 * t23671 * t27091 * t379;
    (t105508, t105510, t105511, t105512, t105514, t105516, t105517, t105518, t105520, t105524)
}
