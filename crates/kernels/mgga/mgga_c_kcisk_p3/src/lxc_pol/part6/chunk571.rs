//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 571/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk571<F: Float>(t1254: F, t7959: F, t4083: F, t7927: F, t4087: F, t6020: F, t7914: F, t7917: F, t7920: F, t2141: F, t1275: F, t4100: F) -> (F, F, F, F, F) {
    let t7960 = t7959 * t1254;
    let t7963 = t7927 * t4083;
    let t7970 = t4087 + F::cast_from(0.61805555555555555556e-2_f64) * t6020 - F::cast_from(0.61805555555555555555e-2_f64) * t7914 + F::cast_from(0.18541666666666666667e-1_f64) * t7917 - F::cast_from(0.92708333333333333333e-2_f64) * t7920;
    let t7976 = t2141 * t2141;
    let t7978 = t4100 * t7976 * t1275;
    (t7960, t7963, t7970, t7976, t7978)
}
