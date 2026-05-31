//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1170/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1170<F: Float>(t114: F, t7898: F, t7937: F, t5542: F, t7934: F, t2014: F, t25826: F, t5891: F, t5915: F, t6998: F, t25822: F, t28679: F) -> (F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t29993 = F::cast_from(2.0_f64) * t7898 * t7937;
    let t29996 = t7934 * t5542;
    let t29998 = F::cast_from(2.0_f64) * t2014 * t29996;
    let t29999 = t25826 * t5891;
    let t30001 = t6998 * t5915;
    let t30004 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t25822 + t28679 + t29999 / F::cast_from(4.0_f64) - t30001 / F::cast_from(8.0_f64));
    (t29993, t29996, t29998, t30004)
}
