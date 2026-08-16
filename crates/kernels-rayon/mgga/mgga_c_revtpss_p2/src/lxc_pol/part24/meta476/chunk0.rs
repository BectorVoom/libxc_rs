//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1460/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460(t11262: f64, t3127: f64, t6262: f64, t3160: f64, t65338: f64, t1062: f64, t19463: f64, t15711: f64, t4834: f64, t1041: f64, t6301: f64, t3150: f64, t6307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65596 = t3127 * t11262 * t6262;
    let t65654 = t65338 * t3160;
    let t65717 = t19463 * t1062;
    let t65859 = t4834 * t15711;
    let t66022 = t1041 * t11262 * t6301;
    let t66029 = t3150 * t11262 * t6307;
    (t65596, t65654, t65717, t65859, t66022, t66029)
}
