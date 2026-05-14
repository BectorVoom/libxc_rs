//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 673/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk673<F: Float>(t2975: F, t2984: F, t484: F, t709: F, t712: F, t715: F, t2992: F, t2998: F, t1381: F, t691: F, t1378: F, t75: F, t288: F, t682: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5030 = 80.0 * t2975;
    let t5031 = 0.11696447245269292414e1 * t2984;
    let t5032 = t709 * t484;
    let t5033 = 20.0 * t5032;
    let t5034 = t712 * t484;
    let t5035 = 12.0 * t5034;
    let t5036 = t715 * t484;
    let t5037 = 32.0 * t5036;
    let t5038 = 4.0 * t2992;
    let t5039 = 40.0 * t2998;
    let t5040 = t1381 * t691;
    let t5041 = 0.17315859105681463759e2 * t5040;
    let t5042 = t1378 * t75;
    let t5043 = t5042 * t288;
    let t5044 = 0.11696447245269292414e1 * t5043;
    let t5045 = t1381 * t682;
    (t5030, t5031, t5032, t5033, t5034, t5035, t5036, t5037, t5038, t5039, t5040, t5041, t5042, t5043, t5044, t5045)
}
