//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1277/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1277<F: Float>(t281: F, t555: F, t93238: F, t25898: F, t7303: F, t25917: F, t9303: F, t10073: F, t1444: F, t2029: F, t25929: F, t26041: F, t9664: F) -> (F, F, F, F) {
    let t94849 = t281 * t93238 * t555;
    let t94851 = t94849 * t25898 * t7303;
    let t94854 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t25917;
    let t94857 = t10073 * t25929 * t2029 * t1444;
    let t94865 = F::cast_from(0.46263278077393568556e-2_f64) * t26041 * t9664;
    (t94851, t94854, t94857, t94865)
}
