//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1314/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1314(t100001: f64, t100003: f64, t100005: f64, t100007: f64, t100009: f64, t100011: f64, t100013: f64, t100015: f64, t100017: f64, t100019: f64, t100021: f64, t99984: f64, t99986: f64, t99988: f64, t99990: f64, t99992: f64, t99994: f64, t99997: f64, t99999: f64) -> f64 {
    let t101701 = 0.9375e-1_f64 * t99984 - 0.1875e0_f64 * t99986 + 0.20234375e-1_f64 * t99988 + 0.5e0_f64 * t99990 + 0.28777777777777777778e0_f64 * t99992 - 0.809375e-1_f64 * t99994 - 0.9375e-1_f64 * t99997 - 0.41666666666666666667e-1_f64 * t99999 - 0.25e0_f64 * t100001 - 0.26979166666666666667e-1_f64 * t100003 + 0.41666666666666666667e-1_f64 * t100005 + 0.21583333333333333333e0_f64 * t100007 + 0.53958333333333333333e-1_f64 * t100009 - 0.125e0_f64 * t100011 + 0.20833333333333333333e-1_f64 * t100013 - 0.5625e0_f64 * t100015 + 0.375e0_f64 * t100017 + 0.53958333333333333334e-1_f64 * t100019 - 0.26979166666666666667e-1_f64 * t100021;
    t101701
}
