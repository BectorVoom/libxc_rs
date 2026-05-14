//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1010/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1010<F: Float>(t31643: F, t527: F, t31464: F, t31468: F, t31471: F, t31473: F, t31475: F, t35629: F, t35632: F, t35636: F, t35638: F, t35643: F, t35647: F, t35648: F, t35651: F, t35653: F, t35656: F, t35660: F, t35662: F) -> (F,) {
    let t35664 = t31643 * t527;
    let t35666 = 0.31448092289604152068e-3 * t35629 - t35632 + t35636 - 0.15724046144802076034e-2 * t35638 - 0.20965394859736101378e-3 * t31464 - 0.12579236915841660827e-2 * t31468 - t31471 + t31473 - t31475 / 192.0 + 13.0 / 96.0 * t35643 - t35647 - t35648 + 0.37737710747524982482e-2 * t35651 + t35653 + 0.68765625e-1 * t35656 + 0.916875e-1 * t35660 - 0.2250885951198661191e-1 * t35662 - 0.11337795902333997111e-1 * t35664;
    (t35666,)
}
