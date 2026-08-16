//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1599/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1599(t1484: f64, t258: f64, t776: f64, t23270: f64, t25038: f64, t1527: f64, t2717: f64, t865: f64, t1888: f64, t6547: f64, t7485: f64, t857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25039 = t258 * t1484;
    let t25040 = t25039 * t776;
    let t25041 = t23270 * t25040;
    let t25042 = t25038 * t25041;
    let t25044 = t2717 * t1527;
    let t25045 = t25044 * t865;
    let t25046 = t23270 * t25045;
    let t25047 = t1888 * t25046;
    let t25049 = t6547 * t7485;
    let t25053 = t857 * t1527;
    (t25039, t25040, t25041, t25042, t25044, t25045, t25046, t25047, t25049, t25053)
}
