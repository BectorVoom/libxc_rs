//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1157/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1157<F: Float>(t1923: F, t2047: F, t2048: F, t25117: F, t25150: F, t26172: F, t6954: F, t7352: F, t92628: F, t92632: F, t95230: F, t95241: F, t95243: F, t95246: F, t95248: F, t95253: F) -> F {
    let t95254 = -F::new(8.0) / F::new(3.0) * t95230 + t1923 * t2047 * t92628 / F::new(3.0) - F::new(2.0) * t25117 * t7352 + t92632 * t2048 / F::new(3.0) + t25150 * t7352 + t6954 * t26172 - F::new(8.0) / F::new(3.0) * t95241 - F::new(16.0) / F::new(3.0) * t95243 + F::new(88.0) / F::new(9.0) * t95246 + F::new(16.0) / F::new(3.0) * t95248 - t95253;
    t95254
}
