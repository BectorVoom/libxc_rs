//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 938/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk938(t18502: f64, t2599: f64, t4917: f64, t766: f64, t9791: f64, t2606: f64, t11593: f64, t14114: f64, t18455: f64, t18457: f64, t18461: f64, t18464: f64, t18468: f64, t18473: f64, t18476: f64, t18479: f64, t18483: f64, t18488: f64, t18493: f64, t18499: f64, t1901: f64, t446: f64) -> (f64, f64) {
    let t18503 = t2599 * t18502;
    let t18506 = t4917 * t766;
    let t18507 = t9791 * t18506;
    let t18508 = t2606 * t18507;
    let t18511 = -2.0_f64 / 27.0_f64 * t18455 - 2.0_f64 / 27.0_f64 * t18457 + 2.0_f64 / 27.0_f64 * t1901 * t18461 + 4.0_f64 / 9.0_f64 * t1901 * t18464 - 4.0_f64 / 27.0_f64 * t1901 * t18468 + t1901 * t18473 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t18476 + 2.0_f64 / 9.0_f64 * t1901 * t18479 + 2.0_f64 / 9.0_f64 * t446 * t18483 + t446 * t18488 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t18493 + 8.0_f64 / 27.0_f64 * t14114 - 8.0_f64 / 9.0_f64 * t11593 * t18499 - 2.0_f64 / 9.0_f64 * t1901 * t18503 - 2.0_f64 / 9.0_f64 * t1901 * t18508;
    (t18506, t18511)
}
