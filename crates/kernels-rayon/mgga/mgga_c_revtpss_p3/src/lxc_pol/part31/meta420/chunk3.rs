//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1513/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1513(t11134: f64, t11304: f64, t15189: f64, t15209: f64, t15210: f64, t15211: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> f64 {
    let t18950 = -t11304 - 4.0_f64 / 27.0_f64 * t11134 - 8.0_f64 / 27.0_f64 * t15189 + t15209 - t15210 + t15211 + 2.0_f64 / 27.0_f64 * t18919 - 10.0_f64 / 27.0_f64 * t18906 + 4.0_f64 / 3.0_f64 * t18911 - 4.0_f64 / 9.0_f64 * t18915 - 2.0_f64 / 9.0_f64 * t18924 - 2.0_f64 * t18928 + 4.0_f64 / 3.0_f64 * t18932 + t18934 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t18939 + 2.0_f64 / 3.0_f64 * t18944 - t18948 / 3.0_f64;
    t18950
}
