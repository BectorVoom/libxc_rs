//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1001/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1001(t15195: f64, t4266: f64, t1240: f64, t2766: f64, t4141: f64, t5410: f64, t8392: f64, t1212: f64, t2842: f64, t4181: f64, t15460: f64, t5415: f64) -> (f64, f64, f64, f64, f64) {
    let t19497 = t15195 * t4266;
    let t19500 = t2766 * t1240;
    let t19501 = t19500 * t4141;
    let t19504 = t8392 * t5410;
    let t19506 = t2842 * t1212;
    let t19507 = t19506 * t4181;
    let t19508 = t15460 * t19507;
    let t19511 = t8392 * t5415;
    (t19497, t19501, t19504, t19508, t19511)
}
