//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1413/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1413<F: Float>(t3799: F, t3853: F, t3858: F, t12267: F, t1340: F, t3719: F, t550: F, t1995: F, t67: F, t246: F, t3734: F, t3777: F, t3802: F) -> (F, F, F, F, F, F, F, F) {
    let t12388 = t3799 * t3853;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    let t12407 = t550 * t3719;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12420 = t550 * t3734;
    let t12429 = t3777 * t3802;
    (t12388, t12395, t12397, t12407, t12418, t12419, t12420, t12429)
}
