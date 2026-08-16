//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 669/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk669<F: Float>(t37: F, t4888: F, t89: F, t1422: F, t458: F, t1419: F, t425: F, t1416: F, t44: F, t48: F, t35: F, t40: F) -> (F, F, F, F, F, F, F) {
    let t4889 = t37 * t4888;
    let t4890 = t4889 * t89;
    let t4891 = F::cast_from(120.0_f64) * t4890;
    let t4896 = t1422 * t458;
    let t4898 = t1419 * t425;
    let t4900 = t1416 * t458;
    let t4901 = F::cast_from(60.0_f64) * t4900;
    let t4902 = t44 * t44;
    let t4904 = F::cast_from(1.0_f64) / t48 / t4902;
    let t4911 = t35 * t40;
    (t4889, t4891, t4896, t4898, t4901, t4904, t4911)
}
