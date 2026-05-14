//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1323/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1323<F: Float>(t1651: F, t1969: F, t27165: F, t5899: F, t23649: F, t27059: F, t2087: F, t23671: F, t27157: F, t5916: F, t920: F, t26888: F, t379: F, t23667: F, t1039: F, t2075: F, t2185: F, t23657: F, t5900: F) -> (F, F, F, F, F, F, F) {
    let t105541 = t5899 * t1969 * t27165 * t1651;
    let t105543 = t23649 * t27059;
    let t105544 = 2.0 * t105543;
    let t105548 = t27157 * t23671 * t5916 * t920 * t2087;
    let t105550 = t26888 * t379;
    let t105552 = t5899 * t23667 * t105550;
    let t105557 = t23657 * t2185 * t5900 * t1039 * t2075;
    (t105541, t105543, t105544, t105548, t105550, t105552, t105557)
}
