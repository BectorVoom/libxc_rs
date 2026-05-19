//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 946/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk946<F: Float>(t10973: F, t2300: F, t265: F, t267: F, t10645: F) -> (F, F, F, F) {
    let t10974 = F::cast_from(0.30487649791575028314e-3_f64) * t10973;
    let t10976 = t2300 * t265;
    let t10977 = t10976 * t267;
    let t10978 = t10645 * t10977;
    (t10974, t10976, t10977, t10978)
}
