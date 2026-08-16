//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 891/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk891(t2902: f64, t3145: f64, t4349: f64, t2497: f64, t3366: f64, t8045: f64, t9260: f64, t13001: f64, t1382: f64, t605: f64, t12862: f64, t10298: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42470 = 6.0_f64 * t4349 * t2902 * t3145;
    let t42473 = 12.0_f64 * t4349 * t3366 * t2497;
    let t42475 = 2.0_f64 * t8045 * t9260;
    let t42478 = 2.0_f64 * t1382 * t13001 * t605;
    let t42481 = 6.0_f64 * t4349 * t12862 * t605;
    let t42483 = 4.0_f64 * t6556 * t10298;
    (t42470, t42473, t42475, t42478, t42481, t42483)
}
