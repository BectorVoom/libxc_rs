//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 737/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk737(t312: f64, t9570: f64, t4142: f64, t8392: f64, t9577: f64, t1882: f64, t4252: f64, t1225: f64, t8232: f64, t309: f64, t799: f64, t4152: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15386 = t312 * t9570;
    let t15400 = 4.0_f64 / 81.0_f64 * t8392 * t4142;
    let t15402 = t312 * t9577;
    let t15419 = 2.0_f64 / 9.0_f64 * t1882 * t4252;
    let t15420 = t8232 * t1225;
    let t15460 = t799 * t309;
    let t15467 = 2.0_f64 / 27.0_f64 * t8392 * t4152;
    (t15386, t15400, t15402, t15419, t15420, t15460, t15467)
}
