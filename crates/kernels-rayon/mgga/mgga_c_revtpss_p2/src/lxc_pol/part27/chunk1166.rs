//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1166/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1166(t2142: f64, t3738: f64, t26969: f64, t3566: f64, t26936: f64, t7642: f64, t1203: f64, t1208: f64, t487: f64, t3790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26970 = t2142 * t3738;
    let t26971 = t26969 * t26970;
    let t26976 = t3566 * t2142;
    let t26979 = t7642 * t26936;
    let t26982 = t1203 * t1203;
    let t26983 = t26982 * t1208;
    let t26984 = t26983 * t487;
    let t26987 = t2142 * t3790;
    (t26970, t26971, t26976, t26979, t26982, t26983, t26984, t26987)
}
