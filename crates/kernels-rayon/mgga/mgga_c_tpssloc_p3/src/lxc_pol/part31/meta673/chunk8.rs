//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2032/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2032(t1375: f64, t1385: f64, t16439: f64, t1843: f64, t20023: f64, t20044: f64, t26996: f64, t27062: f64, t29360: f64, t3887: f64, t5321: f64, t6460: f64, t7194: f64, t7213: f64, t7214: f64, t7925: f64, t93341: f64, t97640: f64, t97644: f64, t97647: f64) -> f64 {
    let t102900 = 4.0_f64 * t5321 * t27062 + 4.0_f64 * t16439 * t7925 - t20044 * t7214 + 2.0_f64 * t1375 * t3887 * t7213 * t6460 - t7194 * t20023 + 4.0_f64 * t5321 * t26996 + 2.0_f64 * t1375 * t3887 * t29360 * t1385 - 2.0_f64 * t93341 * t1843 + 0.3289868133696452873e-1_f64 * t97640 + 0.6579736267392905746e-1_f64 * t97644 + 0.6579736267392905746e-1_f64 * t97647;
    t102900
}
