//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 841/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk841<F: Float>(t3034: F, t5: F, t736: F, t5307: F, t5321: F, t5327: F, t7685: F, t7688: F, t7689: F, t7691: F, t7694: F, t7699: F, t7701: F, t7707: F) -> F {
    let t8908 = t3034 * t5;
    let t8909 = t8908 * t736;
    let t8912 = t5307 + t5321 + F::cast_from(0.1350520664e0_f64) * t5327 - F::cast_from(0.23392894490538584828e1_f64) * t7685 + t7688 + F::cast_from(0.69263436422725855035e2_f64) * t7689 + F::cast_from(0.34631718211362927518e2_f64) * t7691 - F::cast_from(0.8103123984e0_f64) * t7694 + F::cast_from(0.2701041328e0_f64) * t7699 - F::cast_from(0.54217906501508699211e-2_f64) * t8909 + F::cast_from(24.0_f64) * t7701 - t7707;
    t8912
}
