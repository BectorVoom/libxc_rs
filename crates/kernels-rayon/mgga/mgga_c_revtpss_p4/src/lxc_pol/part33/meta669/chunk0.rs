//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2195/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195(t4173: f64, t4187: f64, t21698: f64, t603: f64, t5816: f64, t640: f64, t77: f64, t29561: f64, t644: f64, t4241: f64, t7705: f64, t1927: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108813 = t4173 * t4187;
    let t108816 = t603 * t21698;
    let t108864 = t77 * t640 * t5816;
    let t108872 = t77 * t29561 * t644;
    let t108876 = t77 * t7705 * t4241;
    let t108879 = t1927 * t5816;
    (t108813, t108816, t108864, t108872, t108876, t108879)
}
