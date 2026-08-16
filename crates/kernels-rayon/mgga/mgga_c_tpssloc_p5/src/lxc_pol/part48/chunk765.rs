//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 765/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk765(t12030: f64, t12033: f64, t12444: f64, t1375: f64, t1386: f64, t2092: f64, t22639: f64, t22650: f64, t24064: f64, t24071: f64, t24082: f64, t24088: f64, t24092: f64, t24095: f64, t3758: f64, t3882: f64, t3889: f64, t3912: f64, t568: f64, t7194: f64, t7199: f64, t7214: f64) -> f64 {
    let t24098 = t24064 * t568 + 4.0_f64 * t3758 * t7199 + 4.0_f64 * t3882 * t7199 + 0.6579736267392905746e-1_f64 * t22639 - t24071 - 2.0_f64 * t3758 * t7214 - 2.0_f64 * t3882 * t7214 - t7194 * t3912 - t12030 * t2092 - t12033 * t2092 + 0.16449340668482264365e-1_f64 * t22650 - 2.0_f64 * t12444 * t2092 - 2.0_f64 * t24082 * t1386 + 2.0_f64 * t7194 * t3889 + 2.0_f64 * t1375 * t24088 - 6.0_f64 * t1375 * t24092 - 2.0_f64 * t24095 * t1386;
    t24098
}
