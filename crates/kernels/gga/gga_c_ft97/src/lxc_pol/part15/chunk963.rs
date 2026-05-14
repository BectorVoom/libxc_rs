//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 963/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk963<F: Float>(t4883: F, t1073: F, t12137: F, t20023: F, t2258: F, t2265: F, t2266: F, t2281: F, t39417: F, t39430: F, t39431: F, t39487: F, t39495: F, t39603: F, t39616: F, t4454: F, t4458: F, t4462: F, t4872: F, t631: F, t637: F, t639: F, t65166: F, t65258: F, t85469: F, t85483: F, t8633: F, t8634: F, t8680: F, t87975: F, t87994: F, t88010: F) -> (F,) {
    let t88016 = t4883 * t4883;
    let t88021 = 2.0 * t631 * t2258 * t8634 * t85469 + 10.0 / 3.0 * t65166 - t39603 - 16.0 / 27.0 * t2265 * t39487 * t20023 * t1073 + 4.0 / 9.0 * t2265 * t12137 * t85483 + 6.0 * t2265 * t8680 * t4462 * t4872 + 2.0 * t2265 * t39495 * t4454 * t4872 + 4.0 * t2265 * t2266 * t4458 * t4883 + 14.0 / 81.0 * t631 * t39430 * t39431 * t85469 - 8.0 / 9.0 * t631 * t8633 * t39417 * t85469 + 10.0 / 9.0 * t65258 - 30.0 * t631 * t637 * t39616 * t87975 + t631 * t637 * t639 * (t87994 + t88010) / 2.0 - 9.0 / 2.0 * t631 * t637 * t2281 * t88016;
    (t88021,)
}
