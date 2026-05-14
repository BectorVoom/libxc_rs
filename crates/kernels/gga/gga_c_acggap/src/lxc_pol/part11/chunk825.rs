//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 825/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk825<F: Float>(t30219: F, t7867: F, t7871: F, t1165: F, t3346: F, t604: F, t7493: F, t2070: F, t30792: F, t2067: F, t4198: F, t30267: F, t3360: F, t7643: F, t30225: F, t438: F) -> (F, F, F, F, F, F, F, F) {
    let t30846 = t30219 * t7867;
    let t30848 = t30219 * t7871;
    let t30852 = t7493 * t1165 * t604 * t3346;
    let t30854 = t30792 * t2070;
    let t30856 = t4198 * t2067;
    let t30861 = t3360 * t30267;
    let t30862 = t30861 * t7643;
    let t30866 = t30225 * t438;
    (t30846, t30848, t30852, t30854, t30856, t30861, t30862, t30866)
}
