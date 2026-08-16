//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2792/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2792(t10090: f64, t122: f64, t14144: f64, t2482: f64, t6861: f64, t72: f64, t9994: f64, t14145: f64, t4114: f64, t10014: f64, t22336: f64, t1398: f64, t73820: f64) -> (f64, f64, f64, f64) {
    let t75035 = t2482 * t10090 * t6861 * t9994 * t72 * t122 * t14144;
    let t75039 = t2482 * t4114 * t6861 * t14145;
    let t75041 = t10014 * t22336;
    let t75047 = t73820 * t1398;
    (t75035, t75039, t75041, t75047)
}
