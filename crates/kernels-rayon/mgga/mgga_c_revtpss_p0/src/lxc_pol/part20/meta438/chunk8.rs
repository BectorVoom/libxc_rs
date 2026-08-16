//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1659/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1659(t12504: f64, t12511: f64, t435: f64, t44009: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t45015: f64, t45023: f64, t45026: f64, t45029: f64, t45033: f64, t45037: f64, t45040: f64, t45231: f64, t45244: f64) -> f64 {
    let t45251 = -t44096 - t44100 + t44103 - t44106 - t44108 + t44111 + t44114 - 0.310907e-1_f64 * (t45231 + t45244) * t435 - 0.19751673498613801407e-1_f64 * t44009 + t45015 - t45023 + t45026 + t45029 - t45033 - t45037 - t45040 - 24.0_f64 * t12511 * t12504;
    t45251
}
