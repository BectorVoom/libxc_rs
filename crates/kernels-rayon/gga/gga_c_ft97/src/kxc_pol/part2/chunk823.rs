//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 823/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk823(t1039: f64, t2087: f64, t91: f64, t9252: f64, t2086: f64, t3526: f64, t590: f64, t2120: f64, t3491: f64, t12574: f64, t12577: f64, t12580: f64, t12584: f64, t12589: f64, t12592: f64, t12918: f64, t9062: f64) -> (f64, f64, f64, f64) {
    let t12921 = t91 * t9252 * t1039 * t2087;
    let t12923 = t2086 * t3526;
    let t12925 = t91 * t12923 * t590;
    let t12928 = t91 * t3491 * t2120;
    let t12937 = -t12918 + t12921 / 8.0_f64 - t12925 / 6.0_f64 - t12928 / 12.0_f64 + 2.0_f64 / 9.0_f64 * t12574 + 8.0_f64 / 9.0_f64 * t12577 - 2.0_f64 / 27.0_f64 * t12580 + 2.0_f64 / 3.0_f64 * t12584 - 2.0_f64 * t12589 + 4.0_f64 / 27.0_f64 * t12592 - 2.0_f64 / 27.0_f64 * t9062;
    (t12921, t12925, t12928, t12937)
}
