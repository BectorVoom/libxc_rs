//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2669/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2669(t15618: f64, t15682: f64, t1062: f64, t53877: f64, t15827: f64, t19878: f64, t15711: f64, t4834: f64, t11672: f64, t19785: f64, t1045: f64, t4772: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65823 = t15618 * t15682;
    let t65837 = t53877 * t1062;
    let t65840 = t19878 * t15827;
    let t65859 = t4834 * t15711;
    let t65892 = t11672 * t19785;
    let t65894 = t1045 * t4772;
    (t65823, t65837, t65840, t65859, t65892, t65894)
}
