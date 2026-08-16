//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 848/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk848(t186: f64, t3873: f64, t1268: f64, t495: f64, t5392: f64, t814: f64, t1459: f64, t171: f64) -> (f64, f64, f64, f64) {
    let t14651 = 1.0_f64 / t3873 / t186;
    let t14974 = t495 * t1268;
    let t15026 = t5392 * t814;
    let t15386 = t171 * t1459;
    (t14651, t14974, t15026, t15386)
}
