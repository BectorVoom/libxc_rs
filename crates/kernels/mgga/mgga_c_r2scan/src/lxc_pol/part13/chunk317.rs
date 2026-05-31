//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 317/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk317<F: Float>(t322: F, t1070: F, t263: F, t321: F, t819: F) -> (F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t1072 = -t819 * t1070 - t263 * t321;
    let t1073 = t1072 / F::cast_from(8.0_f64);
    let t1074 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t1073);
    (t1073, t1074)
}
