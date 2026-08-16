//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 921/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk921<F: Float>(t31643: F, t409: F, t1103: F, t7746: F, t7637: F, t7709: F, t2113: F, t7610: F, t2082: F, t30567: F, t7528: F, t2109: F) -> (F, F, F, F, F, F, F) {
    let t31644 = t31643 * t409;
    let t31646 = t7746 * t1103;
    let t31658 = t7637 * t7709;
    let t31660 = t7610 * t2113;
    let t31662 = t30567 * t2082;
    let t31663 = F::cast_from(0.38586616306262763276e-2_f64) * t31662;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    (t31644, t31646, t31658, t31660, t31663, t31682, t31684)
}
