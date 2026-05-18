//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1045/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1045<F: Float>(t4137: F, t553: F, t303: F, t1489: F, t1494: F, t1497: F, t27387: F, t1464: F, t2642: F, t5653: F, t7923: F, t1394: F) -> (F, F, F, F, F, F, F, F) {
    let t27419 = t553 * t4137;
    let t27420 = t303 * t27419;
    let t27423 = t1494 * t1489 * t1497;
    let t27424 = t27387 * t27423;
    let t27425 = t1464 * t27424;
    let t27427 = t5653 * t2642;
    let t27428 = t7923 * t27427;
    let t27429 = t1394 * t27428;
    (t27419, t27420, t27423, t27424, t27425, t27427, t27428, t27429)
}
