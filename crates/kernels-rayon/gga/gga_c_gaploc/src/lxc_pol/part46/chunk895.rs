//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 895/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk895(t42522: f64, t1365: f64, t31558: f64, t6525: f64, t12963: f64, t1358: f64, t2299: f64, t488: f64, t7888: f64, t9199: f64, t2321: f64, t35215: f64, t9074: f64) -> (f64, f64, f64, f64, f64) {
    let t42523 = 4.0_f64 * t42522;
    let t42529 = t6525 * t1365 * t31558;
    let t42533 = t1358 * t2299 * t12963 * t488;
    let t42537 = 0.94850022118920498663e-2_f64 * t1358 * t7888 * t9199;
    let t42539 = t9074 * t35215 * t2321;
    (t42523, t42529, t42533, t42537, t42539)
}
