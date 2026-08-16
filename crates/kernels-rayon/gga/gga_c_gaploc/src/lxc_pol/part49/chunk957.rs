//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 957/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk957(t40567: f64, t40570: f64, t2902: f64, t3145: f64, t4349: f64, t2497: f64, t3366: f64, t8045: f64, t9260: f64, t12862: f64, t605: f64, t10298: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42460 = 0.25561950635947166451e1_f64 * t40567;
    let t42461 = 0.29792074959875355558e-1_f64 * t40570;
    let t42470 = 6.0_f64 * t4349 * t2902 * t3145;
    let t42473 = 12.0_f64 * t4349 * t3366 * t2497;
    let t42475 = 2.0_f64 * t8045 * t9260;
    let t42481 = 6.0_f64 * t4349 * t12862 * t605;
    let t42483 = 4.0_f64 * t6556 * t10298;
    (t42460, t42461, t42470, t42473, t42475, t42481, t42483)
}
