//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1084/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1084(t1263: f64, t6573: f64, t1038: f64, t6593: f64, t1244: f64, t1241: f64, t5273: f64, t5292: f64, t1260: f64, t6601: f64, t140: f64, t6652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21093 = t1263 * t6573;
    let t21100 = t6593 * t1038;
    let t21101 = t1244 * t21100;
    let t21102 = t1241 * t21101;
    let t21107 = t5273 * t5292;
    let t21143 = t6601 * t1260;
    let t21169 = t140 * t6652;
    (t21093, t21101, t21102, t21107, t21143, t21169)
}
