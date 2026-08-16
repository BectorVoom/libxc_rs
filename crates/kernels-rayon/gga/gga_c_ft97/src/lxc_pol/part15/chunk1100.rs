//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1100/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1100(t4883: f64, t1073: f64, t12137: f64, t20023: f64, t2258: f64, t2265: f64, t2266: f64, t2281: f64, t39417: f64, t39430: f64, t39431: f64, t39487: f64, t39495: f64, t39603: f64, t39616: f64, t4454: f64, t4458: f64, t4462: f64, t4872: f64, t631: f64, t637: f64, t639: f64, t65166: f64, t65258: f64, t85469: f64, t85483: f64, t8633: f64, t8634: f64, t8680: f64, t87975: f64, t87994: f64, t88010: f64) -> f64 {
    let t88016 = t4883 * t4883;
    let t88021 = 2.0_f64 * t631 * t2258 * t8634 * t85469 + 10.0_f64 / 3.0_f64 * t65166 - t39603 - 16.0_f64 / 27.0_f64 * t2265 * t39487 * t20023 * t1073 + 4.0_f64 / 9.0_f64 * t2265 * t12137 * t85483 + 6.0_f64 * t2265 * t8680 * t4462 * t4872 + 2.0_f64 * t2265 * t39495 * t4454 * t4872 + 4.0_f64 * t2265 * t2266 * t4458 * t4883 + 14.0_f64 / 81.0_f64 * t631 * t39430 * t39431 * t85469 - 8.0_f64 / 9.0_f64 * t631 * t8633 * t39417 * t85469 + 10.0_f64 / 9.0_f64 * t65258 - 30.0_f64 * t631 * t637 * t39616 * t87975 + t631 * t637 * t639 * (t87994 + t88010) / 2.0_f64 - 9.0_f64 / 2.0_f64 * t631 * t637 * t2281 * t88016;
    t88021
}
