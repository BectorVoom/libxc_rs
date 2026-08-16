//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 697/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk697(t13506: f64, t7226: f64, t2508: f64, t12555: f64, t12558: f64, t12561: f64, t12564: f64, t12566: f64, t12569: f64, t471: f64, t12580: f64, t3603: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13507 = t7226 * t13506;
    let t13509 = 0.46143157380853345701e-1_f64 * t2508 * t13507;
    let t13516 = -3.0_f64 / 128.0_f64 * t12555 - 27.0_f64 / 4096.0_f64 * t12558 + 27.0_f64 / 262144.0_f64 * t12561 - 9.0_f64 / 262144.0_f64 * t12564 + 9.0_f64 / 4096.0_f64 * t12566 + t12569 / 128.0_f64;
    let t13517 = t13516 * t471;
    let t13520 = 9.0_f64 / 128.0_f64 * t12555;
    let t13521 = 9.0_f64 / 4096.0_f64 * t12558;
    let t13522 = 3.0_f64 / 4096.0_f64 * t12566;
    let t13523 = 3.0_f64 / 128.0_f64 * t12569;
    let t13524 = 4.0_f64 * t12580;
    let t13535 = t954 * t3603;
    (t13507, t13509, t13516, t13517, t13520, t13521, t13522, t13523, t13524, t13535)
}
