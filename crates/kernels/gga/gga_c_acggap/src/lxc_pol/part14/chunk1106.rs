//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1106/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1106<F: Float>(t2030: F, t361: F, t9700: F, t142: F, t5506: F, t599: F, t2060: F, t9704: F, t1165: F, t5969: F, t604: F, t7493: F) -> (F, F, F, F) {
    let t39330 = t2030 * t361 * t9700;
    let t39334 = t2030 * t142 * t599 * t5506;
    let t39337 = t2060 * t361 * t9704;
    let t39343 = t7493 * t1165 * t604 * t5969;
    (t39330, t39334, t39337, t39343)
}
