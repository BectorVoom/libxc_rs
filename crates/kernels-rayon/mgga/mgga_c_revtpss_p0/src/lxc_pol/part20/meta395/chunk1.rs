//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1452/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1452(t2918: f64, t2924: f64, t2926: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64) -> (f64, f64, f64) {
    let t41510 = t2918 * t2918;
    let t41513 = 0.48245938496077605201e2_f64 * t2924 * t41510 * t2926;
    let t41520 = 0.96141975308641975307e-1_f64 * t41306;
    let t41525 = 0.74166666666666666668e-1_f64 * t41308 + 0.2225e0_f64 * t41312 - 0.33375e0_f64 * t41316 + 0.55625000000000000001e-1_f64 * t41320 + 0.22249999999999999999e0_f64 * t41323 - 0.18541666666666666666e-1_f64 * t41327 + t41520 - 0.24722222222222222222e-1_f64 * t41330 - 0.16481481481481481482e-1_f64 * t41332 + 0.12361111111111111111e-1_f64 * t41334 + 0.13734567901234567901e-1_f64 * t41336;
    (t41510, t41513, t41525)
}
