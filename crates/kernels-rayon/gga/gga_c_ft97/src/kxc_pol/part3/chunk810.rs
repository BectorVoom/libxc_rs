//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 810/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk810(t86: f64, t112: f64, t113: f64, t1577: f64, t16563: f64, t16573: f64, t16579: f64, t3297: f64, t4628: f64, t4635: f64, t5: f64, t502: f64, t505: f64, t992: f64) -> f64 {
    let t87 = 10000000.0_f64 <= t86;
    let t16584 = piecewise3(t87, 0.0_f64, t5 * t16563 * t113 / 4.0_f64 + t5 * t4628 * t505 / 4.0_f64 + t5 * t3297 * t992 / 2.0_f64 - t5 * t16573 * t1577 + t5 * t502 * t4635 / 4.0_f64 + t5 * t112 * t16579 / 4.0_f64);
    t16584
}
