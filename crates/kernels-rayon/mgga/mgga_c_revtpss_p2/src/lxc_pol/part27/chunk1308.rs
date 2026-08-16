//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1308/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1308(t3566: f64, t7627: f64, t7642: f64, t96873: f64, t26948: f64, t487: f64, t8945: f64, t26936: f64, t3736: f64, t7635: f64, t1203: f64, t1294: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97019 = t3566 * t7627;
    let t97034 = t7642 * t96873;
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    let t97050 = t26948 * t26936;
    let t97065 = t7635 * t3736;
    let t97066 = t3566 * t97065;
    let t97067 = t1203 * t1294;
    (t97019, t97034, t97041, t97050, t97066, t97067)
}
