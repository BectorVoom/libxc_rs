//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 326/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk326(t388: f64, t66: f64, t4: f64, t40: f64, t39: f64, t12: f64, t51: f64) -> (f64, f64, f64, f64) {
    let t1669 = t388 * t66;
    let t1689 = t40 * t4;
    let t1690 = t39 * t1689;
    let t1701 = t51 * t12;
    (t1669, t1689, t1690, t1701)
}
