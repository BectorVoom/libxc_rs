//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1020/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1020<F: Float>(t322: F, t12601: F, t1074: F, t2944: F, t1013: F, t3633: F, t2941: F, t11066: F, t11897: F, t1300: F, t327: F, t3373: F, t6693: F, t834: F) -> (F, F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t12602 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t12601);
    let t12611 = t1074 * t2944;
    let t12614 = t3633 * t1013;
    let t12617 = t1074 * t2941;
    let t12622 = -F::cast_from(0.64e0_f64) * t12602 * t327 - F::cast_from(0.256e1_f64) * t11897 * t1013 - F::cast_from(0.384e1_f64) * t11066 * t2944 - F::cast_from(0.128e1_f64) * t3373 * t2941 - F::cast_from(0.384e1_f64) * t6693 * t12611 - F::cast_from(0.256e1_f64) * t1300 * t12614 - F::cast_from(0.128e1_f64) * t1300 * t12617 - F::cast_from(0.64e0_f64) * t834 * t12602;
    (t12602, t12611, t12614, t12617, t12622)
}
