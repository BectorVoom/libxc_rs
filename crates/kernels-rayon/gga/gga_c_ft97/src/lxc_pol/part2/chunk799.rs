//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 799/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk799(t12573: f64, t446: f64, t11003: f64, t569: f64, t3281: f64, t11034: f64, t2205: f64, t2075: f64, t3342: f64, t28: f64, t89: f64, t1017: f64, t1986: f64, t7368: f64) -> (f64, f64, f64, f64, f64) {
    let t12574 = t446 * t12573;
    let t12576 = t569 * t11003;
    let t12577 = t3281 * t12576;
    let t12579 = t2205 * t11034;
    let t12580 = t446 * t12579;
    let t12582 = t3342 * t2075;
    let t12584 = t89 * t28 * t12582;
    let t12587 = t7368 * t1017 * t1986;
    (t12574, t12577, t12580, t12584, t12587)
}
