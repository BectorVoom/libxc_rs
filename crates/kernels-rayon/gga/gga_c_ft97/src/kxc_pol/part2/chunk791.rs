//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 791/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk791(t3404: f64, t549: f64, t2030: f64, t3383: f64, t2057: f64, t1014: f64, t1995: f64, t51: f64, t538: f64, t6: f64, t398: f64, t527: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12418 = t549 * t3404;
    let t12422 = t3383 * t2030;
    let t12425 = t2057 * t3404;
    let t12435 = t1995 * t1014;
    let t12437 = t538 * t6 * t51;
    let t12438 = t12437 * t398;
    let t12441 = t527 * t1014;
    (t12418, t12422, t12425, t12435, t12438, t12441)
}
