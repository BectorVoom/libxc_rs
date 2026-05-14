//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1073/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1073<F: Float>(t730: F, t9348: F, t1976: F, t3604: F, t2874: F, t1954: F, t723: F, t2873: F, t7299: F, t2746: F, t2783: F, t3525: F, t5734: F, t1850: F, t3551: F, t5522: F, t5783: F, t7357: F, t7420: F, t9138: F, t9140: F, t9143: F, t9148: F, t9163: F, t9165: F, t9172: F, t9174: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9350 = 0.23392894490538584828e1 * t730 * t9348;
    let t9351 = t1976 * t3604;
    let t9352 = t9351 * t2874;
    let t9354 = 0.17315859105681463759e2 * t730 * t9352;
    let t9355 = t1954 * t3604;
    let t9356 = t9355 * t723;
    let t9358 = 0.11696447245269292414e1 * t730 * t9356;
    let t9359 = t2873 * t7299;
    let t9361 = 0.34631718211362927518e2 * t730 * t9359;
    let t9363 = 2.0 * t2746 * t2783;
    let t9365 = 2.0 * t5734 * t3525;
    let t9367 = 1.0 * t1850 * t3551;
    let t9378 = 0.142419375e1 * t9138 - 0.1898925e1 * t9140 - 0.9494625e0 * t9143 + 0.1898925e1 * t9165 - t5783 + 0.39862222222222222223e0 * t5522 + 0.79724444444444444445e0 * t7357 - t7420 - 0.29896666666666666667e0 * t9148 + 0.8969e0 * t9163 - 0.76790625e-1 * t9172 + 0.3071625e0 * t9174;
    (t9350, t9351, t9352, t9354, t9355, t9356, t9358, t9359, t9361, t9363, t9365, t9367, t9378)
}
