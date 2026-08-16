//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1091/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1091(t358: f64, t7281: f64, t1882: f64, t34710: f64, t102678: f64, t102682: f64, t102862: f64, t103491: f64, t110: f64, t11490: f64, t11854: f64, t138221: f64, t138223: f64, t138254: f64, t138283: f64, t138285: f64, t138298: f64, t145035: f64, t1871: f64, t1901: f64, t25598: f64, t26166: f64, t26198: f64, t26202: f64, t26240: f64, t26436: f64, t26441: f64, t3113: f64, t3114: f64, t3189: f64, t3204: f64, t3214: f64, t32545: f64, t3266: f64, t3271: f64, t446: f64, t452: f64, t5710: f64, t60426: f64, t7229: f64, t8557: f64, t91771: f64) -> f64 {
    let t146866 = t7281 * t358;
    let t146892 = t1882 * t34710;
    let t146918 = 2.0_f64 / 3.0_f64 * t138221 + 2.0_f64 / 9.0_f64 * t138223 + t446 * t452 * t32545 * t3214 / 3.0_f64 - t1901 * t8557 * t146866 * t3204 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t11854 * t146866 * t3113 - 4.0_f64 / 9.0_f64 * t1901 * t102862 * t26202 - 4.0_f64 / 9.0_f64 * t1901 * t102678 * t26436 + 4.0_f64 / 27.0_f64 * t1901 * t102682 * t26441 - 2.0_f64 / 9.0_f64 * t1901 * t91771 * t26198 + 2.0_f64 / 9.0_f64 * t138254 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26240 + t146892 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t138283 - t138285 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t110 * t145035 + 4.0_f64 * t1901 * t103491 * t7229 * t3266 + 8.0_f64 / 3.0_f64 * t1901 * t60426 * t7229 * t3271 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t26166 * t25598 + t1901 * t138298 * t3114 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t138298 * t3189;
    t146918
}
