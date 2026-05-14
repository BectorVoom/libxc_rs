//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1036/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1036<F: Float>(t2139: F, t3655: F, t2138: F, t3670: F, t3596: F, t3598: F, t3594: F, t7616: F, t479: F, t3089: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26821 = 0.95275595817932748827e-4 * t2139 * t3655;
    let t26824 = t3670 * t2138;
    let t26842 = t3596 * sigma2;
    let t26843 = t26842 * t3598;
    let t26844 = t3594 * t26843;
    let t26848 = t7616 * t3598;
    let t26849 = t3594 * t26848;
    let t26865 = sigma2 * t479;
    let t26866 = t26865 * t3089;
    (t26821, t26824, t26842, t26843, t26844, t26848, t26849, t26865, t26866)
}
