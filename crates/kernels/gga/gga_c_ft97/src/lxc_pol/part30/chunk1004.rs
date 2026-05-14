//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1004/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1004<F: Float>(t342: F, t36086: F, t630: F, t1466: F, t2336: F, t36074: F, t10915: F, t142527: F, t142537: F, t142539: F, t142558: F, t142566: F, t142576: F, t142577: F, t1476: F, t15567: F, t193: F, t231: F, t24989: F, t28719: F, t28828: F, t28832: F, t28837: F, t2917: F, t34291: F, t343: F, t3691: F, t3700: F, t4162: F, t6210: F, t6340: F, t666: F, t684: F, t6963: F, t7022: F, t7084: F) -> (F,) {
    let t153641 = t342 * t630 * t36086;
    let t153646 = t1466 * t2336 * t36074;
    let t153664 = t6963 * t6340 / 3.0 + t1466 * t28837 / 3.0 + t15567 * t2917 * t1476 * t3700 / 6.0 - t15567 * t10915 * t1476 * t3691 / 9.0 + t1466 * t28828 / 3.0 + t1466 * t28832 / 3.0 + t6210 * t7084 / 3.0 - t153641 / 12.0 - t142527 / 54.0 + t142537 - t142539 / 12.0 - t153646 / 54.0 - t1466 * t193 * t24989 * t4162 + t6963 * t34291 / 18.0 - t342 * t343 * t231 * t28719 / 4.0 + t1466 * t666 * t7022 * t684 / 18.0 + t142558 / 18.0 - t142566 / 36.0 - t142576 + t142577 / 18.0;
    (t153664,)
}
