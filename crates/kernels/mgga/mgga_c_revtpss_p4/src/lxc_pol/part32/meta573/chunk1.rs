//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1898/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1898<F: Float>(t28837: F, t3920: F, t1358: F, t212: F, t28888: F, t689: F, t25898: F, t8099: F, t94849: F, t26277: F, t97916: F, t97799: F) -> (F, F, F, F, F) {
    let t102122 = t28837 * t3920;
    let t102129 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t28888 * t1358;
    let t102131 = t94849 * t25898 * t8099;
    let t102133 = t97916 * t26277;
    let t102135 = t97799 * t26277;
    (t102122, t102129, t102131, t102133, t102135)
}
