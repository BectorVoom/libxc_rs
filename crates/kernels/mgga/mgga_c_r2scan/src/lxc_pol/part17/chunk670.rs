//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 670/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk670<F: Float>(t4973: F, t735: F, t1422: F, t425: F, t1510: F, t410: F, t4911: F, t89: F, t36: F, t409: F, t1385: F, t732: F) -> (F, F, F, F, F, F, F) {
    let t4974 = t735 * t4973;
    let t4975 = F::cast_from(0.21687162600603479684e-1_f64) * t4974;
    let t4976 = t1422 * t425;
    let t4978 = t410 * t1510;
    let t4979 = F::new(12.0) * t4978;
    let t4980 = t4911 * t89;
    let t4981 = F::new(24.0) * t4980;
    let t4982 = t36 * t409;
    let t4983 = t4982 * t89;
    let t4987 = t732 * t1385;
    (t4975, t4976, t4979, t4981, t4982, t4983, t4987)
}
