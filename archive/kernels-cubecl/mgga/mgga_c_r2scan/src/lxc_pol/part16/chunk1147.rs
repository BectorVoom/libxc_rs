//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1147/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1147<F: Float>(t1013: F, t1074: F, t11066: F, t11894: F, t12602: F, t12611: F, t12614: F, t12617: F, t1300: F, t19203: F, t2394: F, t2941: F, t2944: F, t3370: F, t3633: F, t3638: F, t6693: F, t829: F, t9676: F, t9693: F) -> F {
    let t42592 = -F::cast_from(0.384e1_f64) * t11066 * t9693 - F::cast_from(0.384e1_f64) * t6693 * t3370 * t2944 - F::cast_from(0.256e1_f64) * t1300 * t11894 * t1013 - F::cast_from(0.256e1_f64) * t1300 * t3633 * t2394 - F::cast_from(0.128e1_f64) * t1300 * t3370 * t2941 - F::cast_from(0.128e1_f64) * t1300 * t1074 * t9676 - F::cast_from(0.128e1_f64) * t1300 * t12602 * t829 - F::cast_from(0.768e1_f64) * t6693 * t3638 * t2394 - F::cast_from(0.1536e2_f64) * t19203 * t12611 * t829 - F::cast_from(0.768e1_f64) * t6693 * t12614 * t829 - F::cast_from(0.384e1_f64) * t6693 * t12617 * t829;
    t42592
}
