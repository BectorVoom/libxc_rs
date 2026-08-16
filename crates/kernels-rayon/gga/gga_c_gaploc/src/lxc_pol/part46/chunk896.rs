//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 896/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk896(t42539: f64, t1358: f64, t3394: f64, t4385: f64, t9078: f64, t10256: f64, t30204: f64, t6525: f64, t488: f64, t9065: f64, t25718: f64, t9194: f64) -> (f64, f64, f64, f64, f64) {
    let t42540 = 0.23712505529730124666e-2_f64 * t42539;
    let t42544 = 0.22131671827748116354e-1_f64 * t1358 * t9078 * t3394 * t4385;
    let t42546 = t6525 * t30204 * t10256;
    let t42547 = 0.47425011059460249332e-2_f64 * t42546;
    let t42551 = 0.31616674039640166221e-2_f64 * t1358 * t9065 * t3394 * t488;
    let t42570 = 0.37940008847568199464e-1_f64 * t1358 * t25718 * t9194;
    (t42540, t42544, t42547, t42551, t42570)
}
