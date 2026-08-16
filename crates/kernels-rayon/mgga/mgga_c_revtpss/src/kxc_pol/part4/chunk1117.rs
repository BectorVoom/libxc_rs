//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1117/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1117(t13784: f64, t3938: f64, t13783: f64, t3935: f64, t828: f64, t1882: f64, t4003: f64, t1353: f64, t1398: f64, t3957: f64, t5690: f64, t1873: f64, t9741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13785 = t13784 * t3938;
    let t13786 = t13783 * t13785;
    let t13789 = t3935 * t828;
    let t13790 = t1882 * t4003;
    let t13791 = t1353 * t1398;
    let t13792 = t13790 * t13791;
    let t13793 = t13789 * t13792;
    let t13797 = 7.0_f64 / 72.0_f64 * t3957 * t5690;
    let t13798 = t9741 * t1873;
    (t13786, t13789, t13790, t13793, t13797, t13798)
}
