//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1896/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1896(t3917: f64, t96576: f64, t94701: f64, t96204: f64, t25878: f64, t96242: f64, t26359: f64, t9303: f64, t2118: f64, t4153: f64, t116: f64, t26153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96577 = t96576 * t3917;
    let t96584 = 0.51727911450665971904e-3_f64 * t94701 * t96204;
    let t96588 = t25878 * t96242;
    let t96591 = 0.26019841438354088051e-2_f64 * t9303 * t26359;
    let t96633 = t4153 * t2118;
    let t96640 = t116 * t26153;
    (t96577, t96584, t96588, t96591, t96633, t96640)
}
