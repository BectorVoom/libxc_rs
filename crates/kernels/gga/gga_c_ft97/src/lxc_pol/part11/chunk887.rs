//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 887/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk887<F: Float>(t40312: F, t40315: F, t40497: F, t40500: F, t40503: F, t40512: F, t40297: F, t40301: F, t40306: F, t40309: F, t40318: F, t40321: F, t40506: F, t40509: F, t165: F, t39641: F, t39646: F, t39649: F, t39655: F, t39658: F, t40517: F, t40519: F, t40522: F, t40525: F, t40540: F, t40555: F, t40570: F, t515: F, t564: F, t9460: F) -> (F,) {
    let t40575 = 4.0 / 27.0 * t40312;
    let t40576 = 8.0 / 81.0 * t40315;
    let t40579 = 56.0 / 243.0 * t40497;
    let t40580 = 8.0 / 27.0 * t40500;
    let t40581 = 4.0 / 9.0 * t40503;
    let t40584 = 8.0 / 9.0 * t40512;
    let t40585 = 20.0 / 27.0 * t40297 - 8.0 / 27.0 * t40301 + 4.0 / 3.0 * t40306 - 4.0 / 3.0 * t40309 - t40575 - t40576 + 2.0 / 27.0 * t40318 + 20.0 / 243.0 * t40321 + t40579 + t40580 - t40581 + 2.0 / 9.0 * t40506 + 4.0 / 3.0 * t40509 + t40584;
    let t40590 = -12.0 * t39641 - 4.0 * t564 * t9460 + 16.0 * t39646 + 12.0 * t39649 + 48.0 * t39655 - 72.0 * t39658 - 2.0 * t40517 - 8.0 * t40519 - 8.0 * t40522 + 24.0 * t40525 - t515 * (t40540 + t40555 + t40570 + t40585) * t165;
    (t40590,)
}
