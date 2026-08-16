//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1308/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1308(t19817: f64, t63844: f64, t14076: f64, t60960: f64, t17930: f64, t44329: f64, t3683: f64, t821: f64, t1398: f64, t2116: f64, t1364: f64, t1991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63845 = t19817 * t63844;
    let t63847 = t60960 * t14076;
    let t63850 = t17930 * t44329;
    let t63859 = t3683 * t821;
    let t63860 = t17930 * t63859;
    let t63863 = t1398 * t2116;
    let t63864 = t17930 * t63863;
    let t63873 = t1991 * t1364;
    (t63845, t63847, t63850, t63859, t63860, t63863, t63864, t63873)
}
