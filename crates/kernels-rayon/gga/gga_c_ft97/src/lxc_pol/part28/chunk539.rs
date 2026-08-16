//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 539/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk539(t5537: f64, t7837: f64, t51: f64, t5566: f64, t1608: f64, t35: f64, t428: f64, t5568: f64, t5567: f64, t5596: f64, t409: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22597 = t7837 * t5537;
    let t22602 = t5566 * t51;
    let t22603 = t1608 * t22602;
    let t22604 = t35 * t428;
    let t22613 = t7837 * t5568;
    let t22619 = t1608 * t5596 * t5567;
    let t22623 = t64 * t409;
    (t22597, t22602, t22603, t22604, t22613, t22619, t22623)
}
