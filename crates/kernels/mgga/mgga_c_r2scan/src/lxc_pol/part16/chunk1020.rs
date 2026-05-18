//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1020/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1020<F: Float>(t322: F, t12601: F, t1074: F, t2944: F, t1013: F, t3633: F, t2941: F, t11066: F, t11897: F, t1300: F, t327: F, t3373: F, t6693: F, t834: F) -> (F, F, F, F, F) {
    let t324 = F::new(0.0) < t322;
    let t12602 = piecewise3::<f64>(t324, F::new(0.0), t12601);
    let t12611 = t1074 * t2944;
    let t12614 = t3633 * t1013;
    let t12617 = t1074 * t2941;
    let t12622 = -F::new(0.64e0) * t12602 * t327 - F::new(0.256e1) * t11897 * t1013 - F::new(0.384e1) * t11066 * t2944 - F::new(0.128e1) * t3373 * t2941 - F::new(0.384e1) * t6693 * t12611 - F::new(0.256e1) * t1300 * t12614 - F::new(0.128e1) * t1300 * t12617 - F::new(0.64e0) * t834 * t12602;
    (t12602, t12611, t12614, t12617, t12622)
}
