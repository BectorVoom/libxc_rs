//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 992/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk992<F: Float>(t322: F, t11893: F, t3633: F, t833: F, t3638: F, t829: F, t1013: F, t3370: F, t1074: F, t2394: F, t11063: F, t11066: F, t1300: F, t2400: F, t327: F, t3373: F, t6693: F, t834: F) -> (F, F, F) {
    let t324 = F::new(0.0) < t322;
    let t11894 = piecewise3::<F>(t324, F::new(0.0), t11893);
    let t11897 = t3633 * t833;
    let t11906 = t3638 * t829;
    let t11909 = t3370 * t1013;
    let t11912 = t1074 * t2394;
    let t11915 = t3633 * t829;
    let t11920 = -F::new(0.64e0) * t11894 * t327 - F::new(0.128e1) * t11897 * t829 - F::new(0.128e1) * t11063 * t1013 - F::new(0.384e1) * t11066 * t2400 - F::new(0.128e1) * t3373 * t2394 - F::new(0.384e1) * t6693 * t11906 - F::new(0.128e1) * t1300 * t11909 - F::new(0.128e1) * t1300 * t11912 - F::new(0.128e1) * t1300 * t11915 - F::new(0.64e0) * t834 * t11894;
    (t11894, t11897, t11920)
}
