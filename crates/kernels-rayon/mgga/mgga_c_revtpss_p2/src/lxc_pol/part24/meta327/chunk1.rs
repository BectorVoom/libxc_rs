//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1137/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1137(t225: f64, t22917: f64, t22923: f64, t22927: f64, t22933: f64, t22813: f64, t9880: f64, t5651: f64, t6816: f64, t1394: f64, t22809: f64, t1877: f64, t1879: f64, t539: f64, t541: f64, t5650: f64, t6832: f64, t6837: f64, t6840: f64) -> (f64, f64, f64, f64, f64) {
    let t22936 = (t22917 + t22923 + t22927 + t22933) * t225;
    let t22944 = t9880 * t22813;
    let t22947 = t5651 * t6816;
    let t22950 = t1394 * t22809;
    let t22953 = -36.0_f64 * t1877 * t6837 + 9.0_f64 * t1877 * t6840 + 9.0_f64 * t1879 * t6832 - t22936 * t541 + 60.0_f64 * t22944 * t539 - 36.0_f64 * t22947 * t5650 + 3.0_f64 * t22950 * t539;
    (t22936, t22944, t22947, t22950, t22953)
}
