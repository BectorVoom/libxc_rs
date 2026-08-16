//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 539/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk539<F: Float>(t322: F, t1010: F, t1276: F, t2378: F, t2924: F, t2928: F, t2938: F, t321: F, t819: F) -> (F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t2940 = -F::cast_from(2.0_f64) * t2378 * t1010 + F::cast_from(2.0_f64) * t1276 * t2928 + t2924 * t321 - t819 * t2938;
    let t2941 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t2940);
    (t2940, t2941)
}
