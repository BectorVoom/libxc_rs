//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3240/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3240(t10301: f64, t10309: f64, t1497: f64, t21809: f64, t2242: f64, t2247: f64, t22656: f64, t22659: f64, t22742: f64, t4173: f64, t4241: f64, t45963: f64, t45972: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t85141: f64, t85177: f64, t85206: f64, t85300: f64) -> f64 {
    let t85305 = -12.0_f64 * t4173 * t21809 - 120.0_f64 * t45963 * t22656 + 840.0_f64 * t45972 * t22656 * t644 - 360.0_f64 * t10309 * t5816 * t4241 + 60.0_f64 * t10301 * t22659 - 360.0_f64 * t10309 * t22659 * t644 + 60.0_f64 * t2247 * t4241 * t5872 + 60.0_f64 * t2247 * t1497 * t21809 - 4.0_f64 * t2242 * t22742 + 20.0_f64 * t2247 * t22742 * t644 - 4.0_f64 * t603 * (t85141 + t85177 + t85206 + t85300);
    t85305
}
