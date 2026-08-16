//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1399/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1399(t121774: f64, t870: f64, t1914: f64, t4303: f64, t2752: f64, t33465: f64, t193: f64, t200: f64, t8565: f64, t115009: f64, t118393: f64, t1408: f64, t1877: f64, t24191: f64, t25: f64, t25015: f64, t25021: f64, t25373: f64, t26744: f64, t26756: f64, t30767: f64, t31430: f64, t31448: f64, t33466: f64, t33476: f64, t606: f64, t6671: f64, t7114: f64, t81547: f64, t98064: f64) -> (f64, f64, f64, f64, f64) {
    let t121775 = t121774 * t870;
    let t121779 = t1914 * t4303;
    let t121782 = t33465 * t2752;
    let t121789 = t193 * t200 * t8565;
    let t121798 = t1877 * t31430 * t1408 / 2.0_f64 - t1877 * t7114 * t118393 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t81547 * t33476 + t26756 * t98064 * t31448 + t1877 * t121775 * t25 / 2.0_f64 + t26756 * t25373 * t121779 - t1877 * t121782 * t6671 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t115009 * t25021 + 3.0_f64 * t121789 * t25015 + t1877 * t33466 * t606 / 2.0_f64 - t1877 * t26744 * t30767 / 2.0_f64;
    (t121775, t121779, t121782, t121789, t121798)
}
