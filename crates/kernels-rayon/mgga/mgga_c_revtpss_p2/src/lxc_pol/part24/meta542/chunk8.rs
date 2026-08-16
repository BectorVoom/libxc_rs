//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1601/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1601(t10309: f64, t13272: f64, t1497: f64, t21663: f64, t2247: f64, t22656: f64, t22659: f64, t22742: f64, t4173: f64, t45972: f64, t5816: f64, t5872: f64, t60224: f64, t603: f64, t60673: f64, t85037: f64, t87072: f64, t87086: f64, t87092: f64, t87195: f64, t87221: f64, t91: f64) -> f64 {
    let t87225 = t87072 * t91 - 16.0_f64 * t85037 * t1497 + 120.0_f64 * t60673 * t5816 - 24.0_f64 * t21663 * t5872 - 480.0_f64 * t60224 * t22656 + 240.0_f64 * t13272 * t22659 - 16.0_f64 * t4173 * t22742 + 840.0_f64 * t45972 * t87086 - 720.0_f64 * t10309 * t5816 * t5872 + 60.0_f64 * t2247 * t87092 + 80.0_f64 * t2247 * t1497 * t22742 - 4.0_f64 * t603 * (t87195 + t87221);
    t87225
}
