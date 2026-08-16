//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 942/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk942<F: Float>(t32003: F, t33535: F, t4210: F, t32130: F, t7965: F, t2131: F, t2147: F, t309: F, t8392: F, t1411: F, t7932: F, t7963: F) -> (F, F, F, F) {
    let t33538 = F::cast_from(0.34694512752820797848e1_f64) * t32003 * t33535 * t4210;
    let t33541 = F::cast_from(0.34694512752820797848e1_f64) * t32130 * t33535 * t7965;
    let t33546 = F::cast_from(0.34694512752820797848e1_f64) * t2131 * t2147 * t8392 * t309;
    let t33551 = t1411 * t309;
    let t33554 = F::cast_from(0.17347256376410398924e1_f64) * t7963 * t7932 * t33551;
    (t33538, t33541, t33546, t33554)
}
