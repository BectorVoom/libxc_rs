//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 733/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk733<F: Float>(t13871: F, t396: F, t12951: F, t403: F, t1390: F, t301: F, t1310: F, t12829: F, t1311: F, t164: F, t25: F, t3951: F) -> (F, F, F, F, F, F) {
    let t13873 = F::cast_from(0.19989765240197019125e-1_f64) * t396 * t13871;
    let t13878 = t403 * t12951;
    let t13893 = F::new(1.0) / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13895 = t403 * t12829;
    let t13900 = t164 * t1311;
    let t13917 = t25 * t3951;
    (t13873, t13878, t13894, t13895, t13900, t13917)
}
