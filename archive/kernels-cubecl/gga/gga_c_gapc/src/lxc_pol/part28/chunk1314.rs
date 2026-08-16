//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1314/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1314<F: Float>(t10343: F, t3734: F, t10336: F, t291: F, t640: F, t3243: F, t6188: F, t10287: F, t11648: F, t24625: F, t3643: F, t11270: F, t24398: F) -> (F, F, F, F, F, F, F) {
    let t35747 = t10343 * t3734;
    let t35749 = t10336 * t3734;
    let t35751 = t640 * t291;
    let t35753 = t3243 * t35751 * t6188;
    let t35755 = t10287 * t11648;
    let t35759 = t3643 * t24625 * t3734;
    let t35762 = t11270 * t24398 * t11648;
    (t35747, t35749, t35751, t35753, t35755, t35759, t35762)
}
