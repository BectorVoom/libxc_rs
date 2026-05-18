//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1111/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1111<F: Float>(t25113: F, t644: F, t77: F, t1927: F, t2315: F, t2247: F, t2259: F, t10406: F, t76: F, t38: F, t45955: F, t2242: F, t2251: F) -> (F, F, F, F, F, F) {
    let t92581 = t77 * t25113 * t644;
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    let t92639 = t2242 * t2251;
    (t92581, t92584, t92588, t92628, t92632, t92639)
}
