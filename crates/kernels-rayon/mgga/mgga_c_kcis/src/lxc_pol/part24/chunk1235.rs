//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1235/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1235(t19745: f64, t2842: f64, t7718: f64, t19807: f64, t1262: f64, t30045: f64, t5329: f64, t6737: f64, t1851: f64, t26996: f64, t5341: f64, t1267: f64, t92735: f64) -> (f64, f64, f64, f64, f64) {
    let t100145 = t2842 * t7718 * t19745;
    let t100148 = t2842 * t7718 * t19807;
    let t100152 = t5329 * t30045 * t6737 * t1262;
    let t100157 = t5329 * t26996 * t1851 * t5341;
    let t100162 = t5329 * t92735 * t6737 * t1267;
    (t100145, t100148, t100152, t100157, t100162)
}
