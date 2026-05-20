//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1034/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1034<F: Float>(t119989: F, t119839: F, t119968: F, t2470: F, t31780: F, t31784: F, t31805: F, t860: F, t817: F, t8485: F, t2718: F, t8479: F) -> (F, F, F, F, F, F) {
    let t119990 = F::cast_from(0.3526350471130277186e-3_f64) * t119989;
    let t119991 = t119968 * t119839;
    let t119993 = t31780 * t2470;
    let t119995 = F::cast_from(0.34270468708064099208e-1_f64) * t31784 * t119993;
    let t120000 = t31805 * t860;
    let t120002 = t120000 * t8485 * t817;
    let t120004 = t8479 * t2718;
    (t119990, t119991, t119993, t119995, t120002, t120004)
}
