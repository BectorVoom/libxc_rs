//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1355/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1355<F: Float>(t3286: F, t4746: F, t1071: F, t3316: F, t342: F, t1647: F, t3298: F, t1089: F, t16183: F, t378: F, t4980: F, t989: F) -> (F, F, F, F, F) {
    let t16502 = t4746 * t3286;
    let t16505 = t3316 * t1071;
    let t16506 = t342 * t16505;
    let t16509 = t1647 * t3298;
    let t16515 = t378 * t16183 * t1089;
    let t16520 = t989 * t4980;
    (t16502, t16506, t16509, t16515, t16520)
}
