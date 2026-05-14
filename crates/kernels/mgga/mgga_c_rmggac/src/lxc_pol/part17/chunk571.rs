//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 571/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk571<F: Float>(t27: F, t8536: F, t2134: F, t2060: F, t8377: F, t1550: F, t1632: F, t645: F, t3928: F, t1635: F, t4044: F, t5898: F, t903: F, t1614: F, t649: F, t2139: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8537 = t27 * t8536;
    let t8538 = t2134 * t8537;
    let t8542 = t2060 * t8377;
    let t8543 = t1550 * t8542;
    let t8545 = t645 * t1632;
    let t8546 = t3928 * t8545;
    let t8548 = t645 * t1635;
    let t8549 = t4044 * t8548;
    let t8551 = t2060 * t5898;
    let t8552 = t903 * t8551;
    let t8561 = t649 * t1614;
    let t8562 = t27 * t8561;
    let t8563 = t2139 * t8562;
    (t8537, t8538, t8542, t8543, t8545, t8546, t8548, t8549, t8551, t8552, t8562, t8563)
}
