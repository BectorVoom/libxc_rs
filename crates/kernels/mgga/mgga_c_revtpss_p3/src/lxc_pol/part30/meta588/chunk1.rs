//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2047/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2047<F: Float>(t1955: F, t7282: F, t9656: F, t25904: F, t94634: F, t94640: F, t281: F, t555: F, t93238: F, t25898: F, t7303: F, t25917: F, t9303: F) -> (F, F, F, F, F, F) {
    let t94823 = t1955 * t7282 * t9656;
    let t94842 = t25904 * t94634;
    let t94844 = t25904 * t94640;
    let t94849 = t281 * t93238 * t555;
    let t94851 = t94849 * t25898 * t7303;
    let t94854 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t25917;
    (t94823, t94842, t94844, t94849, t94851, t94854)
}
