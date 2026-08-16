//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1381/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1381(t1359: f64, t3338: f64, t34411: f64, t6716: f64, t6717: f64, t10409: f64, t31356: f64, t2482: f64, t2792: f64, t9263: f64, t6895: f64, t993: f64) -> (f64, f64, f64, f64, f64) {
    let t34478 = t1359 * t3338;
    let t34484 = 0.69017266717057349418e1_f64 * t6716 * t6717 * t34411;
    let t34485 = t31356 * t10409;
    let t34486 = 0.76685851907841499352e0_f64 * t34485;
    let t34488 = t9263 * t2792 * t2482;
    let t34489 = 0.76685851907841499352e0_f64 * t34488;
    let t34491 = t9263 * t993 * t6895;
    (t34478, t34484, t34486, t34489, t34491)
}
