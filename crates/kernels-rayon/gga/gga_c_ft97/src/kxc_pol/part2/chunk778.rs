//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 778/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk778(t3614: f64, t8675: f64, t2281: f64, t3653: f64, t637: f64, t643: f64, t1073: f64, t2282: f64, t8618: f64, t632: f64, t72: f64, t1075: f64, t8640: f64) -> (f64, f64, f64, f64, f64) {
    let t12190 = 2.0_f64 / 27.0_f64 * t8675 * t3614;
    let t12191 = t2281 * t3653;
    let t12193 = t637 * t12191 * t643;
    let t12198 = t637 * t8618 * t1073 * t2282;
    let t12201 = t72 * t632;
    let t12204 = t8640 * t1075;
    (t12190, t12193, t12198, t12201, t12204)
}
