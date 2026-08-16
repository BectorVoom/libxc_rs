//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1023/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1023<F: Float>(t322: F, t12828: F, t1120: F, t2944: F, t1013: F, t3730: F, t2941: F, t11223: F, t12244: F, t1300: F, t327: F, t3509: F, t6693: F, t834: F) -> (F, F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t12829 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t12828);
    let t12838 = t1120 * t2944;
    let t12841 = t3730 * t1013;
    let t12844 = t1120 * t2941;
    let t12849 = -F::cast_from(0.64e0_f64) * t12829 * t327 - F::cast_from(0.256e1_f64) * t12244 * t1013 - F::cast_from(0.384e1_f64) * t11223 * t2944 - F::cast_from(0.128e1_f64) * t3509 * t2941 - F::cast_from(0.384e1_f64) * t6693 * t12838 - F::cast_from(0.256e1_f64) * t1300 * t12841 - F::cast_from(0.128e1_f64) * t1300 * t12844 - F::cast_from(0.64e0_f64) * t834 * t12829;
    (t12829, t12838, t12841, t12844, t12849)
}
