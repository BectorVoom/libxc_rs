//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1113/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1113(t683: f64, t7640: f64, t10262: f64, t684: f64, t446: f64, t2409: f64, t2682: f64, t10248: f64, t2667: f64, t8232: f64, t10250: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43381 = t683 * t7640;
    let t43382 = t684 * t10262;
    let t43384 = t446 * t43381 * t43382;
    let t43386 = t2409 * t2682;
    let t43388 = t446 * t10248 * t43386;
    let t43390 = t8232 * t2667;
    let t43392 = t1882 * t10250;
    (t43382, t43384, t43386, t43388, t43390, t43392)
}
