//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 953/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk953<F: Float>(t358: F, t7281: F, t1882: F, t34710: F, t102678: F, t102682: F, t102862: F, t103491: F, t110: F, t11490: F, t11854: F, t138221: F, t138223: F, t138254: F, t138283: F, t138285: F, t138298: F, t145035: F, t1871: F, t1901: F, t25598: F, t26166: F, t26198: F, t26202: F, t26240: F, t26436: F, t26441: F, t3113: F, t3114: F, t3189: F, t3204: F, t3214: F, t32545: F, t3266: F, t3271: F, t446: F, t452: F, t5710: F, t60426: F, t7229: F, t8557: F, t91771: F) -> (F,) {
    let t146866 = t7281 * t358;
    let t146892 = t1882 * t34710;
    let t146918 = 2.0 / 3.0 * t138221 + 2.0 / 9.0 * t138223 + t446 * t452 * t32545 * t3214 / 3.0 - t1901 * t8557 * t146866 * t3204 / 9.0 - 2.0 / 9.0 * t1901 * t11854 * t146866 * t3113 - 4.0 / 9.0 * t1901 * t102862 * t26202 - 4.0 / 9.0 * t1901 * t102678 * t26436 + 4.0 / 27.0 * t1901 * t102682 * t26441 - 2.0 / 9.0 * t1901 * t91771 * t26198 + 2.0 / 9.0 * t138254 + 2.0 / 3.0 * t446 * t452 * t5710 * t26240 + t146892 / 9.0 - 2.0 / 9.0 * t138283 - t138285 / 9.0 + 2.0 / 3.0 * t446 * t1871 * t110 * t145035 + 4.0 * t1901 * t103491 * t7229 * t3266 + 8.0 / 3.0 * t1901 * t60426 * t7229 * t3271 - 4.0 / 3.0 * t1901 * t11490 * t26166 * t25598 + t1901 * t138298 * t3114 / 9.0 + 2.0 / 9.0 * t1901 * t138298 * t3189;
    (t146918,)
}
