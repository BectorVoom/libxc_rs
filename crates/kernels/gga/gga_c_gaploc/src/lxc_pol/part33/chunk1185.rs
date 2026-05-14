//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1185/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1185<F: Float>(t30830: F, t7967: F, t913: F, t10609: F, t31054: F, t2754: F, t4130: F, t2482: F, t9272: F, t10608: F, t6895: F, t20671: F, t26328: F, t31037: F, t10241: F, t20550: F) -> (F, F, F, F, F, F) {
    let t35074 = t30830 * t913 * t7967;
    let t35075 = 0.59584149919750711116e-1 * t35074;
    let t35089 = t31054 * t10609;
    let t35090 = 0.11502877786176224903e1 * t35089;
    let t35091 = t4130 * t2754;
    let t35093 = t9272 * t35091 * t2482;
    let t35094 = 0.11502877786176224903e1 * t35093;
    let t35096 = t9272 * t10608 * t6895;
    let t35097 = 0.57514388930881124514e0 * t35096;
    let t35099 = t31037 * t20671 * t26328;
    let t35100 = 0.2556195063594716645e0 * t35099;
    let t35101 = t20550 * t10241;
    (t35075, t35090, t35094, t35097, t35100, t35101)
}
