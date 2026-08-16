//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1495/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1495(t3572: f64, t5002: f64, t3523: f64, t5005: f64, t5019: f64, t5024: f64, t11147: f64, t11778: f64, t3490: f64, t4993: f64, t248: f64, t3521: f64, t4733: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15446 = t5002 * t3572 / 2304.0_f64;
    let t15448 = t5005 * t3523 / 3456.0_f64;
    let t15450 = t5019 * t3572 / 432.0_f64;
    let t15452 = t5024 * t3523 / 648.0_f64;
    let t15453 = t11778 * t11147;
    let t15484 = t3490 * t4993 / 3456.0_f64;
    let t15486 = t248 * t3521 * t4733;
    (t15446, t15448, t15450, t15452, t15453, t15484, t15486)
}
