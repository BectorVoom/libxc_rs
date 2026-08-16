//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1270/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1270(t12: f64, t28885: f64, t11118: f64, t11125: f64, t1151: f64, t1153: f64, t3000: f64, t3005: f64, t30990: f64, t30991: f64, t30993: f64, t30998: f64, t31004: f64, t31005: f64, t31007: f64, t31017: f64, t318: f64, t319: f64, t3706: f64, t3710: f64, t808: f64, t810: f64, t9729: f64, t9738: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t31035 = piecewise3(t84, 0.0_f64, t28885);
    let t31039 = piecewise3(t203, 0.0_f64, (t30990 + t30991 + t30993 + t30998 + t31004 + t31005 + t31007 + t31017) * t319 / 2.0_f64 + t11118 * t810 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t9729 * t1153 + 3.0_f64 / 2.0_f64 * t3706 * t3005 + 3.0_f64 / 2.0_f64 * t3000 * t3710 + 3.0_f64 / 2.0_f64 * t1151 * t9738 + t808 * t11125 / 2.0_f64 + t318 * t31035 / 2.0_f64);
    t31039
}
