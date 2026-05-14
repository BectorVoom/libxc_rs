//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1010/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1010<F: Float>(t24437: F, t30979: F, t2354: F, t4969: F, t6119: F, t6118: F, t5165: F, t6135: F, t24432: F, t1424: F, t5092: F, t743: F, t193: F, t24448: F, t30859: F, t1434: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30980 = t24437 * t30979;
    let t30983 = t2354 * t6119 * t4969;
    let t30984 = t6118 * t30983;
    let t30986 = t6135 * t5165;
    let t30987 = t24432 * t30986;
    let t30988 = t6118 * t30987;
    let t30990 = t1424 * t5092;
    let t30991 = t743 * t30990;
    let t30993 = t24448 * t193 * t30991;
    let t30996 = t743 * t30859;
    let t30998 = t1434 * t193 * t30996;
    (t30980, t30983, t30984, t30986, t30987, t30988, t30990, t30991, t30993, t30996, t30998)
}
