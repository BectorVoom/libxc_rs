//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1823/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823(t1394: f64, t1877: f64, t1879: f64, t22229: f64, t22236: f64, t225: f64, t22809: f64, t22936: f64, t22944: f64, t22947: f64, t22950: f64, t4049: f64, t47171: f64, t539: f64, t541: f64, t5650: f64, t5651: f64, t6816: f64, t6832: f64, t6837: f64, t6840: f64, t91826: f64, t91870: f64, t91875: f64, t91957: f64, t91964: f64, t91967: f64, t91971: f64, t91981: f64, t92017: f64, t92023: f64, t92030: f64) -> f64 {
    let t92063 = -(t91957 + t91964 + t91967 + t91971 + t91981 + t92017 + t92023 + t92030) * t225 * t541 + 12.0_f64 * t22936 * t1879 - 72.0_f64 * t6832 * t6837 + 18.0_f64 * t6832 * t6840 + 240.0_f64 * t1877 * t22944 - 144.0_f64 * t22229 * t22947 + 12.0_f64 * t1877 * t22950 - 360.0_f64 * t539 * t47171 * t91870 + 360.0_f64 * t5650 * t22236 * t6816 - 36.0_f64 * t539 * t4049 * t91875 - 48.0_f64 * t5650 * t5651 * t22809 + 3.0_f64 * t539 * t1394 * t91826;
    t92063
}
