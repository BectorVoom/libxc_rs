//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 924/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk924<F: Float>(t322: F, t12828: F, t1120: F, t2944: F, t1013: F, t3730: F, t2941: F, t11223: F, t12244: F, t1300: F, t327: F, t3509: F, t6693: F, t834: F, t1018: F, t3740: F, t1125: F, t2951: F) -> (F, F, F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t12829 = piecewise3(t324, 0.0, t12828);
    let t12838 = t1120 * t2944;
    let t12841 = t3730 * t1013;
    let t12844 = t1120 * t2941;
    let t12849 = -0.64e0 * t12829 * t327 - 0.256e1 * t12244 * t1013 - 0.384e1 * t11223 * t2944 - 0.128e1 * t3509 * t2941 - 0.384e1 * t6693 * t12838 - 0.256e1 * t1300 * t12841 - 0.128e1 * t1300 * t12844 - 0.64e0 * t834 * t12829;
    let t12851 = t3740 * t1018;
    let t12854 = t1125 * t2951;
    (t12829, t12838, t12841, t12844, t12849, t12851, t12854)
}
