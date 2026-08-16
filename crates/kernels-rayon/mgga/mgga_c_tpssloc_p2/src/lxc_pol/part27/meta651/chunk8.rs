//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2272/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2272(t45844: f64, t6489: f64, t12719: f64, t72: f64, t79: f64, t1410: f64, t9228: f64, t2235: f64, t3961: f64, t3967: f64, t1865: f64, t22519: f64, t22527: f64, t22531: f64, t22537: f64, t22546: f64, t26045: f64, t26048: f64, t26084: f64, t6490: f64, t6495: f64, t7432: f64, t7442: f64, t83814: f64) -> f64 {
    let t90330 = t45844 * t6489;
    let t90334 = t72 * t79 * t12719;
    let t90337 = t9228 * t1410;
    let t90340 = t2235 * t3961;
    let t90343 = t2235 * t3967;
    let t90346 = 5.0_f64 / 3.0_f64 * t26084 * t22527 + 5.0_f64 / 6.0_f64 * t26084 * t22531 + 2.0_f64 / 3.0_f64 * t22519 * t7442 + t22537 * t7442 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6495 * t26045 + 2.0_f64 / 3.0_f64 * t6495 * t26048 - 5.0_f64 / 3.0_f64 * t83814 * t7432 - 5.0_f64 * t90330 * t22546 + 5.0_f64 / 6.0_f64 * t6490 * t90334 + t90337 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t90340 * t1865 + 2.0_f64 / 3.0_f64 * t90343 * t1865;
    t90346
}
