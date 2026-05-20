//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1137/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1137<F: Float>(t1426: F, t3999: F, t2282: F, t55: F, t10309: F, t7565: F, t2139: F, t3655: F, t2138: F, t3670: F, t3596: F, t3598: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t26079 = t1426 * t3999;
    let t26776 = t55 * t2282;
    let t26792 = t10309 * t7565;
    let t26821 = F::cast_from(0.95275595817932748827e-4_f64) * t2139 * t3655;
    let t26824 = t3670 * t2138;
    let t26842 = t3596 * sigma2;
    let t26843 = t26842 * t3598;
    (t26079, t26776, t26792, t26821, t26824, t26842, t26843)
}
