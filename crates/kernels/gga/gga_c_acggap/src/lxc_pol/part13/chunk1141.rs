//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1141/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1141<F: Float>(t2288: F, t4262: F, t7450: F, t922: F, t2310: F, t7780: F, t31643: F, t527: F, t31464: F, t31468: F, t31471: F, t31473: F, t31475: F, t35629: F, t35632: F, t35636: F, t35638: F, t35643: F, t35647: F, t35648: F, t35651: F, t35653: F, t35656: F) -> F {
    let t35660 = t7450 * t4262 * t2288 * t922;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35666 = F::cast_from(0.31448092289604152068e-3_f64) * t35629 - t35632 + t35636 - F::cast_from(0.15724046144802076034e-2_f64) * t35638 - F::cast_from(0.20965394859736101378e-3_f64) * t31464 - F::cast_from(0.12579236915841660827e-2_f64) * t31468 - t31471 + t31473 - t31475 / F::cast_from(192.0_f64) + F::cast_from(13.0_f64) / F::cast_from(96.0_f64) * t35643 - t35647 - t35648 + F::cast_from(0.37737710747524982482e-2_f64) * t35651 + t35653 + F::cast_from(0.68765625e-1_f64) * t35656 + F::cast_from(0.916875e-1_f64) * t35660 - F::cast_from(0.2250885951198661191e-1_f64) * t35662 - F::cast_from(0.11337795902333997111e-1_f64) * t35664;
    t35666
}
