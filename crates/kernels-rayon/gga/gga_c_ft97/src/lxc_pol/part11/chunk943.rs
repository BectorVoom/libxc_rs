//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 943/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk943(t178: f64, t2280: f64, t2282: f64, t2296: f64, t8640: f64, t12116: f64, t12122: f64, t1643: f64, t2265: f64, t2266: f64, t2281: f64, t2294: f64, t3613: f64, t3621: f64, t37315: f64, t37320: f64, t39575: f64, t39603: f64, t39604: f64, t39606: f64, t39608: f64, t39613: f64, t631: f64, t637: f64, t643: f64, t7966: f64, t8654: f64, t8671: f64, t8680: f64) -> f64 {
    let t39616 = 1.0_f64 / t2280 / t178;
    let t39617 = t2282 * t2282;
    let t39622 = t8640 * t2296;
    let t39624 = 8.0_f64 * t2265 * t12116 * t39575 - 4.0_f64 / 3.0_f64 * t2265 * t12122 * t39575 - 8.0_f64 * t2265 * t2266 * t7966 * t643 + 6.0_f64 * t2265 * t3621 * t37315 - 2.0_f64 * t2265 * t3613 * t37320 - 2.0_f64 / 3.0_f64 * t2265 * t8654 * t1643 * t2294 + 12.0_f64 * t2265 * t8680 * t643 * t8671 - t39603 - 4.0_f64 / 3.0_f64 * t39604 - 160.0_f64 / 27.0_f64 * t39606 - 9.0_f64 / 2.0_f64 * t631 * t637 * t2281 * t39608 - 16.0_f64 * t39613 - 30.0_f64 * t631 * t637 * t39616 * t39617 + 10.0_f64 / 3.0_f64 * t39622;
    t39624
}
