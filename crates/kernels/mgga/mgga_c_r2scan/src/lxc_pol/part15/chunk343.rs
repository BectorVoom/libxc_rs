//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 343/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk343<F: Float>(t322: F, t1269: F, t1271: F, t1276: F, t1277: F, t1289: F, t321: F, t819: F, t826: F) -> (F, F) {
    let t324 = F::new(0.0) < t322;
    let t1291 = t1269 * t321 - F::new(2.0) * t1271 * t826 + F::new(2.0) * t1276 * t1277 - t819 * t1289;
    let t1292 = piecewise3::<f64>(t324, F::new(0.0), t1291);
    (t1291, t1292)
}
