//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 349/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk349<F: Float>(t1027: F, t1135: F, t553: F, t894: F, t1110: F, t1111: F, t1116: F, t1121: F, t1125: F, t1131: F, t1133: F) -> (F, F, F, F) {
    let t1136 = t1135 * t1027;
    let t1137 = t1136 * t553;
    let t1138 = t894 * t1137;
    let t1141 = t1110 + t1111 * t1116 / 288.0 + 0.35500316489081544176e-1 * t1121 * t1125 + t1131 + 0.18110753103726578864e-2 * t1133 * t1138;
    (t1136, t1137, t1138, t1141)
}
