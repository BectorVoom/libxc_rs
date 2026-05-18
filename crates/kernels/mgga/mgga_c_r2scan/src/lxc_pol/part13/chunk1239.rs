//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1239/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1239<F: Float>(t322: F, t40814: F, t40850: F, t1013: F, t11063: F, t11897: F, t11909: F, t11912: F, t1292: F, t1295: F, t19203: F, t2394: F, t327: F, t3373: F, t3638: F, t37015: F, t40770: F, t6693: F, t829: F, t834: F, t8398: F) -> (F, F) {
    let t324 = F::new(0.0) < t322;
    let t40851 = t40814 + t40850;
    let t40852 = piecewise3::<f64>(t324, F::new(0.0), t40851);
    let t40869 = -F::new(0.128e1) * t11897 * t1292 - F::new(0.384e1) * t40770 * t1295 - F::new(0.128e1) * t37015 * t1013 - F::new(0.256e1) * t11063 * t2394 - F::new(0.128e1) * t3373 * t8398 - F::new(0.64e0) * t834 * t40852 - F::new(0.768e1) * t6693 * t11909 * t829 - F::new(0.768e1) * t6693 * t11912 * t829 - F::new(0.384e1) * t6693 * t3638 * t1292 - F::new(0.1536e2) * t19203 * t3638 * t1295 - F::new(0.64e0) * t40852 * t327;
    (t40851, t40869)
}
