//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2123/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123(t10523: f64, t1573: f64, t10629: f64, t48096: f64, t47730: f64, t48155: f64, t1556: f64, t2842: f64, t10828: f64, t1580: f64, t2841: f64, t4351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49139 = 0.27595e0_f64 * t48096;
    let t49144 = 0.40256666666666666668e0_f64 * t47730;
    let t49200 = 0.5519e0_f64 * t48155;
    let t49226 = t2842 * t1556;
    let t49263 = t10828 * t1580;
    let t49269 = t4351 * t2841;
    (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269)
}
