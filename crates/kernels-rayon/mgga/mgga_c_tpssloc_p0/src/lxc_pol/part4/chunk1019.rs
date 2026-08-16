//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1019/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1019(t16662: f64, t820: f64, t847: f64, t2697: f64, t5624: f64, t13360: f64, t1516: f64, t5568: f64, t9573: f64, t2563: f64, t5572: f64, t16805: f64, t237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16985 = t847 * t820 * t16662;
    let t16988 = t2697 * t5624;
    let t16990 = t13360 * t1516;
    let t16993 = t9573 * t5568;
    let t16995 = t2563 * t5572;
    let t16997 = t16805 * t237;
    (t16985, t16988, t16990, t16993, t16995, t16997)
}
