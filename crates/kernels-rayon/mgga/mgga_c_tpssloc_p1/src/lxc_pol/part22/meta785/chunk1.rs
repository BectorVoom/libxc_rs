//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2704/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2704(t12680: f64, t1420: f64, t16558: f64, t19368: f64, t19390: f64, t19391: f64, t19394: f64, t19398: f64, t20217: f64, t20234: f64, t2267: f64, t39: f64, t39159: f64, t3966: f64, t3981: f64, t3991: f64, t45970: f64, t45974: f64, t51: f64, t5398: f64, t5416: f64, t607: f64, t68513: f64) -> f64 {
    let t75461 = 5.0_f64 / 36.0_f64 * t45974 * t68513 - 5.0_f64 / 36.0_f64 * t45970 * t68513 - 5.0_f64 / 36.0_f64 * t39 * t19368 * t3966 + 5.0_f64 / 162.0_f64 * t39 * t39159 * t20234 * t607 + 5.0_f64 / 6.0_f64 * t39 * t12680 * t5398 + 5.0_f64 / 6.0_f64 * t39 * t3981 * t16558 + 5.0_f64 / 18.0_f64 * t39 * t2267 * t20217 * t607 + 220.0_f64 / 27.0_f64 * t5416 * t3991 - 40.0_f64 / 9.0_f64 * t1420 * t19394 - 10.0_f64 / 27.0_f64 * t1420 * t19391 - 20.0_f64 / 9.0_f64 * t1420 * t19398 + 5.0_f64 / 36.0_f64 * t51 * t19390 * t3966;
    t75461
}
