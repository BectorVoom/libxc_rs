//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1274/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1274<F: Float>(t11214: F, t11663: F, t6853: F, t760: F, t10343: F, t3734: F, t10336: F, t291: F, t640: F, t3243: F, t6188: F, t10287: F, t11648: F) -> (F, F, F, F, F, F) {
    let t35745 = t11214 * t760 * t6853 * t11663;
    let t35747 = t10343 * t3734;
    let t35749 = t10336 * t3734;
    let t35751 = t640 * t291;
    let t35753 = t3243 * t35751 * t6188;
    let t35755 = t10287 * t11648;
    (t35745, t35747, t35749, t35751, t35753, t35755)
}
