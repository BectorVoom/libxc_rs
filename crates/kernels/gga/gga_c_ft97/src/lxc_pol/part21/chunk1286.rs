//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1286/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1286<F: Float>(t23657: F, t23671: F, t27034: F, t27081: F, t1882: F, t30204: F, t105473: F, t1969: F, t446: F, t920: F, t15625: F, t5916: F, t4417: F, t9049: F, t95379: F, t30244: F, t558: F, t9432: F) -> (F, F, F, F, F, F, F) {
    let t119982 = t23657 * t23671 * t27034 * t27081;
    let t119984 = t1882 * t30204;
    let t119985 = 2.0 / 9.0 * t119984;
    let t119988 = t446 * t1969 * t105473 * t920;
    let t119992 = t446 * t1969 * t5916 * t15625;
    let t119996 = t446 * t9049 * t95379 * t4417;
    let t120000 = t446 * t9432 * t30244 * t558;
    (t119982, t119984, t119985, t119988, t119992, t119996, t120000)
}
