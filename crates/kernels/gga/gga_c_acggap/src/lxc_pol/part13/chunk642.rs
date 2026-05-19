//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 642/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk642<F: Float>(t2975: F, t2984: F, t484: F, t709: F, t712: F, t715: F, t2992: F, t2998: F, t1381: F, t691: F, t1378: F, t75: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5030 = F::new(80.0) * t2975;
    let t5031 = F::cast_from(0.11696447245269292414e1_f64) * t2984;
    let t5032 = t709 * t484;
    let t5033 = F::new(20.0) * t5032;
    let t5034 = t712 * t484;
    let t5035 = F::new(12.0) * t5034;
    let t5036 = t715 * t484;
    let t5037 = F::new(32.0) * t5036;
    let t5038 = F::new(4.0) * t2992;
    let t5039 = F::new(40.0) * t2998;
    let t5040 = t1381 * t691;
    let t5041 = F::cast_from(0.17315859105681463759e2_f64) * t5040;
    let t5042 = t1378 * t75;
    (t5030, t5031, t5033, t5035, t5037, t5038, t5039, t5041, t5042)
}
