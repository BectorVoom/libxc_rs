//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 977/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk977<F: Float>(t11764: F, t3427: F, t11759: F, t11761: F, t869: F, t2645: F, t3769: F, t11905: F, t15473: F, t11822: F, t7522: F, t11825: F, t17891: F, t17899: F, t26416: F, t291: F, t5542: F) -> (F, F, F, F, F, F, F) {
    let t33831 = t11764 * t3427;
    let t33834 = t869 * t11759 * t11761;
    let t33836 = t3769 * t2645;
    let t33838 = t11905 * t15473;
    let t33840 = t11822 * t7522;
    let t33842 = t11825 * t7522;
    let t33847 = t17891 * t5542 * t26416 * t291 * t17899;
    (t33831, t33834, t33836, t33838, t33840, t33842, t33847)
}
