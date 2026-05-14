//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1021/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1021<F: Float>(t11848: F, t11850: F, t869: F, t11854: F, t7553: F, t1078: F, t2387: F, t3756: F, t11764: F, t3427: F, t11759: F, t11761: F, t2645: F, t3769: F, t11905: F, t15473: F) -> (F, F, F, F, F, F, F) {
    let t33823 = t869 * t11848 * t11850;
    let t33825 = t7553 * t11854;
    let t33828 = t2387 * t3756 * t1078;
    let t33831 = t11764 * t3427;
    let t33834 = t869 * t11759 * t11761;
    let t33836 = t3769 * t2645;
    let t33838 = t11905 * t15473;
    (t33823, t33825, t33828, t33831, t33834, t33836, t33838)
}
