//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1875/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1875(t1203: f64, t1208: f64, t487: f64, t2142: f64, t3790: f64, t7652: f64, t2148: f64, t3727: f64, t3566: f64, t7635: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26982 = t1203 * t1203;
    let t26983 = t26982 * t1208;
    let t26984 = t26983 * t487;
    let t26987 = t2142 * t3790;
    let t26988 = t7652 * t26987;
    let t26991 = t2148 * t3727;
    let t26994 = t3566 * t7635;
    (t26982, t26983, t26984, t26988, t26991, t26994)
}
