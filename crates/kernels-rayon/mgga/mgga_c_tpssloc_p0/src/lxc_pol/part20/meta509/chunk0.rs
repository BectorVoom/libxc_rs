//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2025/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2025(t268: f64, t521: f64, t9799: f64, t9847: f64, t677: f64, t9494: f64, t3684: f64, t12110: f64, t9885: f64, t12099: f64, t2663: f64, t12083: f64, t67: f64, t758: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39321 = t521 * t268;
    let t39322 = t9799 * t9847;
    let t39324 = 0.1301229756036208781e0_f64 * t39321 * t39322;
    let t39325 = t677 * t9494;
    let t39327 = 0.38025319932552508021e2_f64 * t3684 * t39325;
    let t39328 = t12110 * t9885;
    let t39330 = t12099 * t2663;
    let t39334 = t12083 * t67 * t758;
    (t39321, t39322, t39324, t39325, t39327, t39328, t39330, t39334)
}
