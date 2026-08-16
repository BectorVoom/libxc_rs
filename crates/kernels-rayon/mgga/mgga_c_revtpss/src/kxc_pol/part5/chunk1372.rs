//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1372/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1372(t21829: f64, t665: f64, t10227: f64, t5895: f64, t658: f64, t1504: f64, t2: f64, t580: f64, t2349: f64, t5823: f64, t9342: f64, t100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21830 = t21829 * t665;
    let t21835 = t10227 * t5895;
    let t21836 = t21835 * t658;
    let t21839 = t1504 * t2;
    let t21840 = t21839 * t580;
    let t21845 = t2349 * t5823;
    let t21846 = t21845 * t658;
    let t21850 = -t580 - 3.0_f64 * t9342;
    let t21851 = t100 * t21850;
    (t21830, t21836, t21840, t21846, t21850, t21851)
}
