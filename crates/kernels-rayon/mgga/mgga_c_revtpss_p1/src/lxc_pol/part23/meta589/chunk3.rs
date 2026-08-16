//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2223/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2223(t11479: f64, t11480: f64, t18919: f64, t18924: f64, t18934: f64, t19002: f64, t19004: f64, t19009: f64, t23521: f64, t23523: f64, t23536: f64, t23538: f64, t23541: f64, t23543: f64) -> f64 {
    let t23693 = 0.20128333333333333333e0_f64 * t18919 - 0.60385000000000000001e0_f64 * t18924 + 0.30192500000000000001e0_f64 * t18934 - t11479 - t11480 + 0.5519e-1_f64 * t19002 - 0.33114e0_f64 * t19004 + 0.16557e0_f64 * t19009 - 0.3883875e1_f64 * t23521 + 0.247573125e0_f64 * t23523 + 0.258925e1_f64 * t23536 + 0.16504875e0_f64 * t23538 + 0.19419375e1_f64 * t23541 - 0.412621875e-1_f64 * t23543;
    t23693
}
