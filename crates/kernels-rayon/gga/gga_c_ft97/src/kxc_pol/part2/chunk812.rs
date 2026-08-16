//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 812/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk812(t12754: f64, t574: f64, t605: f64, t12600: f64, t144: f64, t1060: f64, t1651: f64, t569: f64, t1643: f64, t2205: f64, t2230: f64, t925: f64) -> (f64, f64, f64, f64, f64) {
    let t12756 = t574 * t605 * t12754;
    let t12759 = t144 * t12600;
    let t12763 = t569 * t1060 * t1651;
    let t12767 = t2205 * t1060 * t1643;
    let t12771 = t569 * t2230 * t925;
    (t12756, t12759, t12763, t12767, t12771)
}
