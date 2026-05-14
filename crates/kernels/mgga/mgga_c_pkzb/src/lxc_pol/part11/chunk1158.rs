//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1158/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1158<F: Float>(t3046: F, t9805: F, t3747: F, t7972: F, t3052: F, t9798: F, t11190: F, t2215: F, t836: F, t18427: F, t18445: F, t18554: F, t18555: F, t27262: F, t27292: F, t27295: F, t31067: F, t31088: F, t31204: F, t31206: F, t31208: F, t31210: F, t31213: F, t31216: F) -> (F, F, F, F, F) {
    let t31218 = t9805 * t3046;
    let t31220 = t7972 * t3747;
    let t31222 = t3052 * t9798;
    let t31225 = t2215 * t11190 * t836;
    let t31230 = t18554 - 0.93011851851851851854e0 * t18427 + t18555 - 0.89690000000000000001e0 * t27262 + 0.82156666666666666665e0 * t27292 + 0.11958666666666666667e1 * t27295 - 0.3560484375e1 * t31204 + 0.427258125e1 * t31206 - 0.28483875e1 * t31208 - 0.28483875e1 * t31210 - 0.9494625e0 * t31213 + 0.1151859375e0 * t31216 - 0.230371875e0 * t31218 + 0.46074375e0 * t31220 + 0.46074375e0 * t31222 + 0.15358125e0 * t31225 - 0.29896666666666666667e0 * t31067 + 0.8969e0 * t31088 - 0.7302814814814814815e0 * t18445;
    (t31218, t31220, t31222, t31225, t31230)
}
