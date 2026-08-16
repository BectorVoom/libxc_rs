//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1163/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1163(t2148: f64, t26936: f64, t1203: f64, t7627: f64, t7637: f64, t1294: f64, t7652: f64, t12626: f64, t2147: f64, t7635: f64, t2142: f64, t3568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26937 = t2148 * t26936;
    let t26940 = t7627 * t1203;
    let t26941 = t7637 * t26940;
    let t26944 = t7627 * t1294;
    let t26945 = t7652 * t26944;
    let t26948 = t2147 * t12626;
    let t26949 = t26948 * t7635;
    let t26950 = t2142 * t3568;
    (t26937, t26940, t26941, t26944, t26945, t26948, t26949, t26950)
}
