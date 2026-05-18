//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 327/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk327<F: Float>(t12: F, t1072: F, t1086: F, t1112: F, t1114: F, t1118: F, t1147: F, t135: F, t273: F, t805: F, t972: F, t977: F, t326: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t1151 = t1147 * t135 * t273 * t805 - t1072 + t1086 + t1112 + t1114 - t1118;
    let t1153 = piecewise3::<f64>(t84, F::new(0.0), t972);
    let t1161 = F::new(1.0) / t977;
    let t1162 = t326 * t1161;
    (t1151, t1153, t1162)
}
