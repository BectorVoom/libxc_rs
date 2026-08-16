//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1448/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448(t18353: f64, t2689: f64, t18348: f64, t2710: f64, t2713: f64, t18562: f64, t2626: f64, t2609: f64, t5944: f64, t10815: f64, t5980: f64, t40398: f64, t6024: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62129 = t2689 * t18353;
    let t62251 = t2710 * t2713 * t18348;
    let t62276 = t18562 * t2626;
    let t62300 = t5944 * t2609;
    let t62399 = t10815 * t5980;
    let t62401 = t40398 * t6024;
    (t62129, t62251, t62276, t62300, t62399, t62401)
}
