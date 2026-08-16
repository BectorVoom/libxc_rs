//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1121/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1121(t27051: f64, t539: f64, t1323: f64, t7918: f64, t1385: f64, t7936: f64, t3887: f64, t1375: f64, t1386: f64, t16030: f64, t2092: f64, t24071: f64, t26217: f64, t26335: f64, t26340: f64, t26345: f64, t26352: f64, t26357: f64, t27009: f64, t3882: f64, t568: f64, t7925: f64) -> (f64, f64, f64, f64) {
    let t27052 = t539 * t27051;
    let t27059 = t1323 * t7918;
    let t27061 = t7936 * t1385;
    let t27062 = t3887 * t27061;
    let t27065 = 0.3289868133696452873e-1_f64 * t26217 - t24071 + 2.0_f64 * t3882 * t7925 - t27009 * t1386 - t16030 * t2092 + t27052 * t568 + 0.9869604401089358619e-1_f64 * t26335 + 0.3289868133696452873e-1_f64 * t26340 + 0.82246703342411321825e-2_f64 * t26345 - 0.16449340668482264365e-1_f64 * t26352 + 0.3289868133696452873e-1_f64 * t26357 + t27059 * t568 + 2.0_f64 * t1375 * t27062;
    (t27052, t27059, t27062, t27065)
}
