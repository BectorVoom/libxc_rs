//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 781/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk781<F: Float>(t12037: F, t12052: F, t1974: F, t1964: F, t5396: F, t755: F, t5399: F, t763: F, t12019: F, t10542: F, t10559: F, t10563: F, t10566: F, t10602: F, t10707: F, t10709: F, t10712: F, t10718: F, t10752: F, t10760: F, t11999: F, t12013: F, t12018: F, t12020: F, t1966: F, t5375: F, t764: F) -> F {
    let t12053 = t12037 + t12052;
    let t12054 = t12053 * t1974;
    let t12058 = F::new(1.0) / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = F::new(1.0) / t5399 / t763;
    let t12062 = t12019 * t12061;
    let t12065 = -F::new(6.0) * t11999 * t5375 + t10559 - t10563 - t10707 - t10709 - t10712 + t10718 - t10752 - t10760 + t10602 - F::new(0.3109e-1) * t12013 * t764 - F::new(0.19298809906722418785e3) * t12018 * t12020 + F::new(1.0) * t1966 * t12054 + F::new(0.20691336878655965246e4) * t12059 * t12062 - t10542 + t10566;
    t12065
}
