//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1593/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1593(t10918: f64, t10976: f64, t868: f64, t251: f64, t9646: f64, t22: f64, t780: f64, t2455: f64, t9285: f64, t2454: f64, t2829: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10977 = t10918 + t10976;
    let t10978 = t868 * t10977;
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    let t10984 = 0.19637199382202157274e-3_f64 * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = 0.46263278077393568556e-2_f64 * t2454 * t10985;
    let t10988 = t779 * t2829;
    (t10977, t10978, t10981, t10982, t10984, t10985, t10987, t10988)
}
