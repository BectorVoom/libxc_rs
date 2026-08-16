//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1058/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1058(t17912: f64, t2302: f64, t31443: f64, t3176: f64, t1530: f64, t31056: f64, t13287: f64, t33953: f64, t5136: f64, t5141: f64, t15386: f64, t3073: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34821 = t31443 * t17912 * t2302 * t3176;
    let t34823 = t1530 * t31056;
    let t34826 = t34823 * t13287 * t33953 * t5136;
    let t34828 = t33953 * t5141;
    let t34830 = t34823 * t15386 * t34828;
    let t34833 = t3073 * t31056;
    (t34821, t34823, t34826, t34828, t34830, t34833)
}
