//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 914/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk914<F: Float>(t1843: F, t7069: F, t7064: F, t2617: F, t948: F, t7803: F, t7802: F, t822: F) -> (F, F, F, F, F) {
    let t9760 = t1843 * t7069;
    let t9762 = F::cast_from(0.64087718584518535698e-3_f64) * t7064 * t9760;
    let t9787 = t948 * t2617;
    let t9788 = t7803 * t9787;
    let t9796 = t822 * t7802;
    (t9760, t9762, t9787, t9788, t9796)
}
