//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 639/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk639<F: Float>(t2084: F, t570: F, t27: F, t2145: F, t551: F, t2134: F, t2060: F, t8377: F, t1550: F, t1632: F, t645: F, t3928: F) -> (F, F, F, F, F, F, F, F) {
    let t8532 = t2084 * t570;
    let t8533 = t27 * t8532;
    let t8534 = t2145 * t8533;
    let t8536 = t2084 * t551;
    let t8537 = t27 * t8536;
    let t8538 = t2134 * t8537;
    let t8542 = t2060 * t8377;
    let t8543 = t1550 * t8542;
    let t8545 = t645 * t1632;
    let t8546 = t3928 * t8545;
    (t8533, t8534, t8537, t8538, t8542, t8543, t8545, t8546)
}
