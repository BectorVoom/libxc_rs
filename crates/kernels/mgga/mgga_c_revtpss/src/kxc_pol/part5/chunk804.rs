//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 804/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk804<F: Float>(t114: F, t5915: F, t655: F, t2335: F, t4261: F, t5892: F, t69: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t5916 = t655 * t5915;
    let t5920 = piecewise3(t115, 0.0, t2335 + 2.0 / 3.0 * t4261 + t69 * t5892 / 4.0 - t69 * t5916 / 8.0);
    (t5916, t5920)
}
