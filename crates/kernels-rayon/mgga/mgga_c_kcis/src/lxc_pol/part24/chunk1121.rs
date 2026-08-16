//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1121/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1121(t2388: f64, t2379: f64, t2385: f64, t60: f64, t81: f64, t9260: f64, t684: f64, t9261: f64, t20: f64, t4879: f64, t2840: f64, t4992: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36957 = t2388 * t2388;
    let t36958 = 1.0_f64 / t36957;
    let t36962 = t2379 * t2385;
    let t37000 = t60 / t9260 / t81;
    let t37013 = t684 * t9261;
    let t37041 = t4879 * t20;
    let t42530 = t86 * t4992 * t2840;
    (t36958, t36962, t37000, t37013, t37041, t42530)
}
