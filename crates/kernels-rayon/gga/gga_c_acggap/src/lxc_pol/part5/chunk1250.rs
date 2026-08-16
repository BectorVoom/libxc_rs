//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1250/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1250(t1181: f64, t3361: f64, t5087: f64, t530: f64, t1861: f64, t3670: f64, t13788: f64, t13791: f64, t17619: f64, t17621: f64, t17623: f64, t17627: f64, t17631: f64, t17635: f64, t17650: f64, t17661: f64) -> f64 {
    let t22993 = t3361 * t1181 * t530 * t5087;
    let t22995 = t3670 * t1861;
    let t23003 = 0.85748036236139473944e-3_f64 * t17619 + 0.17149607247227894789e-2_f64 * t17621 + 0.85748036236139473944e-3_f64 * t17623 - 0.51448821741683684367e-2_f64 * t17627 + 0.34299214494455789578e-2_f64 * t22993 - 0.90702367218671976884e-1_f64 * t22995 + 0.40015750243531754508e-2_f64 * t17631 + 0.13719685797782315831e-1_f64 * t17635 + 35.0_f64 / 72.0_f64 * t13788 + 35.0_f64 / 216.0_f64 * t13791 - 0.68598428988911579156e-2_f64 * t17650 - 0.40015750243531754508e-1_f64 * t17661;
    t23003
}
