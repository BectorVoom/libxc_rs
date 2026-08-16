//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1082/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1082(t1882: f64, t34667: f64, t10969: f64, t32414: f64, t1332: f64, t1339: f64, t137843: f64, t137864: f64, t145719: f64, t1825: f64, t1871: f64, t25846: f64, t26001: f64, t26154: f64, t3052: f64, t32120: f64, t32366: f64, t3238: f64, t3255: f64, t32625: f64, t3266: f64, t3281: f64, t34415: f64, t34536: f64, t34647: f64, t379: f64, t446: f64, t447: f64, t452: f64, t488: f64, t499: f64, t5635: f64, t5710: f64, t5743: f64, t5750: f64, t6454: f64, t6564: f64, t7165: f64, t7288: f64, t83: f64, t942: f64, t986: f64) -> (f64, f64) {
    let t146268 = t1882 * t34667;
    let t146329 = t10969 * t32414;
    let t146338 = -t446 * t452 * t32366 * t942 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t146268 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t986 * t32120 + t446 * t452 * t3238 * t32625 / 3.0_f64 - t137843 / 27.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t452 * t5750 * t6454 - t446 * t447 * t34536 * t379 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t499 * t34415 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t7288 * t3266 + 4.0_f64 / 3.0_f64 * t446 * t1871 * t6564 * t5635 + 2.0_f64 / 3.0_f64 * t446 * t452 * t1825 * t34647 + 2.0_f64 / 3.0_f64 * t446 * t452 * t488 * t25846 * t1332 + 2.0_f64 / 3.0_f64 * t446 * t452 * t488 * t6454 * t5743 + 2.0_f64 / 3.0_f64 * t446 * t452 * t5710 * t26154 - 2.0_f64 / 9.0_f64 * t3281 * t447 * t7288 * t3052 + 4.0_f64 / 3.0_f64 * t446 * t83 * t145719 + 4.0_f64 / 3.0_f64 * t446 * t1871 * t1339 * t26001 + 4.0_f64 / 9.0_f64 * t137864 + 2.0_f64 / 3.0_f64 * t446 * t83 * t146329 - 2.0_f64 / 3.0_f64 * t446 * t1871 * t488 * t7165 * t3255;
    (t146329, t146338)
}
