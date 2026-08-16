//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1229/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1229<F: Float>(t137: F, t3074: F, t34509: F, t5126: F, t26578: F, t34503: F, t203: F, t27596: F, t5698: F, t6: F, t1030: F, t144: F, t33521: F, t4052: F) -> (F, F, F, F) {
    let t34535 = t3074 * t137;
    let t34537 = t34509 * t34535 * t5126;
    let t34539 = t34503 * t26578;
    let t34546 = t5698 * t203 * t6 * t27596;
    let t34547 = t1030 * t4052 * t33521 * t144 * t34546;
    (t34535, t34537, t34539, t34547)
}
