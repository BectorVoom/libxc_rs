//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 798/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk798(t140: f64, t12417: f64, t12559: f64, t526: f64, t27: f64, t89: f64, t358: f64, t582: f64, t2999: f64, t1018: f64, t1636: f64, t10998: f64, t569: f64) -> (f64, f64, f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t12561 = piecewise3(t141, t12417 + t12559, 0.0_f64);
    let t12562 = t526 * t12561;
    let t12564 = t89 * t27 * t12562;
    let t12566 = t582 * t358;
    let t12568 = t89 * t2999 * t12566;
    let t12571 = t89 * t1636 * t1018;
    let t12573 = t569 * t10998;
    (t12561, t12564, t12568, t12571, t12573)
}
