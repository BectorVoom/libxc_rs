//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1268/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1268<F: Float>(t3046: F, t7135: F, t1078: F, t1982: F, t3140: F, t3259: F, t378: F, t42859: F, t1032: F, t7150: F, t1071: F, t11239: F) -> (F, F, F, F, F, F) {
    let t93459 = t3046 * t7135;
    let t93464 = t1982 * t3259 * t3140 * t1078;
    let t93469 = t378 * t42859;
    let t93471 = t1982 * t93469 * t1078;
    let t93484 = t3259 * t1032;
    let t93485 = t7150 * t93484;
    let t93488 = t1071 * t11239;
    (t93459, t93464, t93471, t93484, t93485, t93488)
}
