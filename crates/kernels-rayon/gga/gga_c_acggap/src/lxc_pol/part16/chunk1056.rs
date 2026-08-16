//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1056/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1056(t1839: f64, t463: f64, t119: f64, t9767: f64, t2146: f64, t2147: f64, t30036: f64, t33475: f64, t33488: f64, t33489: f64, t33496: f64, t33500: f64, t33504: f64, t33507: f64, t38621: f64, t464: f64, t556: f64, t7890: f64, t7931: f64, t7932: f64, t8004: f64, t8400: f64, t8402: f64, t8993: f64, t9025: f64, t944: f64, t9793: f64) -> f64 {
    let t38685 = t1839 * t463;
    let t38689 = t119 * t9767;
    let t38693 = -0.26020884564615598386e1_f64 * t2146 * t8004 * t9793 * t463 - t33475 - 0.8673628188205199462e0_f64 * t2146 * t7890 * t38621 * t944 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t8993 * t556 - t30036 + t33488 + t33496 - 0.17347256376410398924e1_f64 * t7931 * t33489 * t9025 + 0.8673628188205199462e0_f64 * t8400 * t33489 * t8402 - 0.8673628188205199462e0_f64 * t7931 * t7932 * t38685 - 0.65854491829355115987e0_f64 * t38689 * t464 - t33500 + t33504 + 0.10408353825846239354e2_f64 * t33507;
    t38693
}
