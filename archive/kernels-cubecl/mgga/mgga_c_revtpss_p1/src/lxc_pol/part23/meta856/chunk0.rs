//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2745/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745<F: Float>(t17708: F, t59498: F, t12916: F, t21041: F, t3718: F, t21165: F, t12809: F, t20796: F, t13045: F, t5284: F, t5245: F, t5457: F) -> (F, F, F, F, F, F) {
    let t72011 = t59498 * t17708;
    let t72017 = t3718 * t12916 * t21041;
    let t72064 = t3718 * t12916 * t21165;
    let t72071 = t12809 * t12916 * t20796;
    let t72086 = t13045 * t5284;
    let t72143 = t5457 * t5245;
    (t72011, t72017, t72064, t72071, t72086, t72143)
}
