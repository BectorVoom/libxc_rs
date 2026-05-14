//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1036/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1036<F: Float>(t31570: F, t31593: F, t31598: F, t31602: F, t35775: F, t35785: F, t35789: F, t35795: F, t35798: F, t35800: F, t37719: F, t40145: F, t40147: F, t40152: F, t40156: F, t40158: F, t40163: F) -> (F,) {
    let t40165 = 0.31448092289604152068e-3 * t31570 - 0.21437009059034868486e-3 * t31593 - t31598 - t31602 + t35775 + t35785 + t35789 + t37719 - t35795 + t35798 + t35800 + 0.17149607247227894789e-2 * t40145 + 7.0 / 144.0 * t40147 + 0.10718504529517434243e-3 * t40152 + 0.7145669686344956162e-4 * t40156 - 0.31448092289604152068e-3 * t40158 - 0.20965394859736101379e-3 * t40163;
    (t40165,)
}
