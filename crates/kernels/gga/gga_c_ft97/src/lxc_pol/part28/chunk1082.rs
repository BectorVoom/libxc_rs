//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1082/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1082<F: Float>(t1882: F, t34667: F, t10969: F, t32414: F, t1332: F, t1339: F, t137843: F, t137864: F, t145719: F, t1825: F, t1871: F, t25846: F, t26001: F, t26154: F, t3052: F, t32120: F, t32366: F, t3238: F, t3255: F, t32625: F, t3266: F, t3281: F, t34415: F, t34536: F, t34647: F, t379: F, t446: F, t447: F, t452: F, t488: F, t499: F, t5635: F, t5710: F, t5743: F, t5750: F, t6454: F, t6564: F, t7165: F, t7288: F, t83: F, t942: F, t986: F) -> (F, F) {
    let t146268 = t1882 * t34667;
    let t146329 = t10969 * t32414;
    let t146338 = -t446 * t452 * t32366 * t942 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146268 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t986 * t32120 + t446 * t452 * t3238 * t32625 / F::cast_from(3.0_f64) - t137843 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t5750 * t6454 - t446 * t447 * t34536 * t379 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t499 * t34415 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t7288 * t3266 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t6564 * t5635 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t1825 * t34647 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t488 * t25846 * t1332 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t488 * t6454 * t5743 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t5710 * t26154 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t447 * t7288 * t3052 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t145719 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t1339 * t26001 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t137864 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t146329 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t488 * t7165 * t3255;
    (t146329, t146338)
}
