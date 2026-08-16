//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 992/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk992(t15377: f64, t4139: f64, t4257: f64, t8392: f64, t4262: f64, t10580: f64, t309: f64, t312: f64, t9570: f64, t13863: f64, t2413: f64, t4145: f64) -> (f64, f64, f64, f64, f64) {
    let t15378 = t4139 * t15377;
    let t15382 = 2.0_f64 / 27.0_f64 * t8392 * t4257;
    let t15384 = 2.0_f64 / 27.0_f64 * t8392 * t4262;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    let t15387 = t15386 * t13863;
    let t15388 = t15385 * t15387;
    let t15391 = t4145 * t2413;
    (t15378, t15382, t15384, t15388, t15391)
}
