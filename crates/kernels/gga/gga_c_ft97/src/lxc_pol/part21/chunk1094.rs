//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1094/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1094<F: Float>(t1882: F, t27036: F, t23649: F, t27074: F, t27120: F, t27059: F, t27097: F, t1369: F, t27128: F, t376: F, t1637: F, t6677: F, t89: F, t1900: F, t579: F, t6: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t105482 = t1882 * t27036;
    let t105483 = 2.0 / 9.0 * t105482;
    let t105510 = t23649 * t27074;
    let t105511 = 2.0 / 27.0 * t105510;
    let t105516 = t23649 * t27120;
    let t105517 = 2.0 / 9.0 * t105516;
    let t105543 = t23649 * t27059;
    let t105544 = 2.0 * t105543;
    let t105559 = t1882 * t27097;
    let t105560 = 4.0 / 9.0 * t105559;
    let t105567 = t1369 * t376 * t27128;
    let t105568 = 2.0 / 3.0 * t105567;
    let t105570 = t89 * t1637 * t6677;
    let t105578 = t91 * t579 * t6 * t1900;
    (t105482, t105483, t105510, t105511, t105516, t105517, t105543, t105544, t105559, t105560, t105567, t105568, t105570, t105578)
}
