//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 814/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk814(t1156: f64, t6068: f64, t3383: f64, t3390: f64, t4721: f64, t4770: f64, t5973: f64, t5977: f64, t5981: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64) -> (f64, f64) {
    let t6069 = t6068 * t1156;
    let t6084 = -0.1294625e1_f64 * t5993 + 0.258925e1_f64 * t6000 + t3383 - 0.20128333333333333334e0_f64 * t4721 - 0.20128333333333333333e0_f64 * t5973 + 0.60385e0_f64 * t5977 + 0.301925e0_f64 * t5981 + 0.82524375e-1_f64 * t6006 + 0.16504875e0_f64 * t6008 + t3390 - 0.11038e0_f64 * t4770 - 0.27595e-1_f64 * t6012 + 0.16557e0_f64 * t6015 + 0.82785e-1_f64 * t6018;
    (t6069, t6084)
}
