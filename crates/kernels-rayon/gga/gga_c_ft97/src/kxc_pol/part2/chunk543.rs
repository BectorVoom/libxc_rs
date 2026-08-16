//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 543/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk543(t2210: f64, t3430: f64, t1570: f64, t160: f64, t3188: f64, t157: f64, t2097: f64) -> (f64, f64, f64, f64, f64) {
    let t3431 = t2210 * t3430;
    let t3434 = t160 * t1570;
    let t3435 = t3434 * t3188;
    let t3436 = t2210 * t3435;
    let t3439 = t2097 * t157;
    (t3431, t3434, t3435, t3436, t3439)
}
