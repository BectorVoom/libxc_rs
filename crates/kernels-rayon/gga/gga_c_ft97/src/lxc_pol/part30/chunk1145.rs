//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1145/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1145(t342: f64, t36086: f64, t630: f64, t1466: f64, t2336: f64, t36074: f64, t10915: f64, t142527: f64, t142537: f64, t142539: f64, t142558: f64, t142566: f64, t142576: f64, t142577: f64, t1476: f64, t15567: f64, t193: f64, t231: f64, t24989: f64, t28719: f64, t28828: f64, t28832: f64, t28837: f64, t2917: f64, t34291: f64, t343: f64, t3691: f64, t3700: f64, t4162: f64, t6210: f64, t6340: f64, t666: f64, t684: f64, t6963: f64, t7022: f64, t7084: f64) -> f64 {
    let t153641 = t342 * t630 * t36086;
    let t153646 = t1466 * t2336 * t36074;
    let t153664 = t6963 * t6340 / 3.0_f64 + t1466 * t28837 / 3.0_f64 + t15567 * t2917 * t1476 * t3700 / 6.0_f64 - t15567 * t10915 * t1476 * t3691 / 9.0_f64 + t1466 * t28828 / 3.0_f64 + t1466 * t28832 / 3.0_f64 + t6210 * t7084 / 3.0_f64 - t153641 / 12.0_f64 - t142527 / 54.0_f64 + t142537 - t142539 / 12.0_f64 - t153646 / 54.0_f64 - t1466 * t193 * t24989 * t4162 + t6963 * t34291 / 18.0_f64 - t342 * t343 * t231 * t28719 / 4.0_f64 + t1466 * t666 * t7022 * t684 / 18.0_f64 + t142558 / 18.0_f64 - t142566 / 36.0_f64 - t142576 + t142577 / 18.0_f64;
    t153664
}
