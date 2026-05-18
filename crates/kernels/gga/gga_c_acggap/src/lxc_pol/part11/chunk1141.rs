//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1141/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1141<F: Float>(t2288: F, t4262: F, t7450: F, t922: F, t2310: F, t7780: F, t31643: F, t527: F, t31464: F, t31468: F, t31471: F, t31473: F, t31475: F, t35629: F, t35632: F, t35636: F, t35638: F, t35643: F, t35647: F, t35648: F, t35651: F, t35653: F, t35656: F) -> F {
    let t35660 = t7450 * t4262 * t2288 * t922;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35666 = F::new(0.31448092289604152068e-3) * t35629 - t35632 + t35636 - F::new(0.15724046144802076034e-2) * t35638 - F::new(0.20965394859736101378e-3) * t31464 - F::new(0.12579236915841660827e-2) * t31468 - t31471 + t31473 - t31475 / F::new(192.0) + F::new(13.0) / F::new(96.0) * t35643 - t35647 - t35648 + F::new(0.37737710747524982482e-2) * t35651 + t35653 + F::new(0.68765625e-1) * t35656 + F::new(0.916875e-1) * t35660 - F::new(0.2250885951198661191e-1) * t35662 - F::new(0.11337795902333997111e-1) * t35664;
    t35666
}
