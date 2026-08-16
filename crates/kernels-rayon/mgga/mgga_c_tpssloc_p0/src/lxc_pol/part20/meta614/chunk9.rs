//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2213/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213(t51: f64, t9300: f64, t12606: f64, t12680: f64, t12698: f64, t1409: f64, t1420: f64, t2244: f64, t2250: f64, t2267: f64, t2274: f64, t39: f64, t39159: f64, t39168: f64, t3966: f64, t3981: f64, t3990: f64, t45970: f64, t45971: f64, t607: f64, t9258: f64, t9287: f64, t9288: f64, t9305: f64) -> f64 {
    let t45974 = t51 * t9300;
    let t45977 = -5.0_f64 / 36.0_f64 * t39 * t9287 * t3966 * t2244 + 5.0_f64 / 162.0_f64 * t39 * t39159 * t1409 * t9288 + 5.0_f64 / 6.0_f64 * t39 * t2267 * t12606 * t607 + 5.0_f64 / 6.0_f64 * t39 * t12680 * t2250 + 5.0_f64 / 18.0_f64 * t39 * t3981 * t9258 - 20.0_f64 / 9.0_f64 * t1420 * t9305 + 5.0_f64 / 36.0_f64 * t51 * t9300 * t3966 * t2244 + 5.0_f64 / 162.0_f64 * t51 * t39168 * t1409 * t9288 + 5.0_f64 / 6.0_f64 * t51 * t2274 * t12606 * t607 + 5.0_f64 / 6.0_f64 * t51 * t12698 * t2250 + 5.0_f64 / 18.0_f64 * t51 * t3990 * t9258 - 5.0_f64 / 36.0_f64 * t45970 * t45971 + 5.0_f64 / 36.0_f64 * t45974 * t45971;
    t45977
}
