//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1141/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1141(t2288: f64, t4262: f64, t7450: f64, t922: f64, t2310: f64, t7780: f64, t31643: f64, t527: f64, t31464: f64, t31468: f64, t31471: f64, t31473: f64, t31475: f64, t35629: f64, t35632: f64, t35636: f64, t35638: f64, t35643: f64, t35647: f64, t35648: f64, t35651: f64, t35653: f64, t35656: f64) -> f64 {
    let t35660 = t7450 * t4262 * t2288 * t922;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35666 = 0.31448092289604152068e-3_f64 * t35629 - t35632 + t35636 - 0.15724046144802076034e-2_f64 * t35638 - 0.20965394859736101378e-3_f64 * t31464 - 0.12579236915841660827e-2_f64 * t31468 - t31471 + t31473 - t31475 / 192.0_f64 + 13.0_f64 / 96.0_f64 * t35643 - t35647 - t35648 + 0.37737710747524982482e-2_f64 * t35651 + t35653 + 0.68765625e-1_f64 * t35656 + 0.916875e-1_f64 * t35660 - 0.2250885951198661191e-1_f64 * t35662 - 0.11337795902333997111e-1_f64 * t35664;
    t35666
}
