//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 988/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk988<F: Float>(t1490: F, t987: F, t3382: F, t5192: F, t1016: F, t1524: F, t4667: F, t997: F, t1451: F, t3670: F, t4999: F, t935: F) -> (F, F, F, F, F, F) {
    let t16359 = t987 * t1490;
    let t16373 = t3382 * t5192;
    let t16375 = t1016 * t1524;
    let t16388 = t997 * t4667;
    let t16390 = t3670 * t1451;
    let t16392 = t935 * t4999;
    (t16359, t16373, t16375, t16388, t16390, t16392)
}
