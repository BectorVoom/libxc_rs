//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 748/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk748<F: Float>(t15292: F, t15294: F, t15296: F, t15298: F, t15302: F, t15304: F, t15306: F, t15308: F, t891: F, t898: F, t2977: F, t85: F) -> (F, F) {
    let t15310 = -F::cast_from(0.69046666666666666667e1_f64) * t15292 + F::cast_from(0.23015555555555555556e1_f64) * t15294 - F::cast_from(0.26851481481481481482e1_f64) * t15296 - F::cast_from(0.93932222222222222223e0_f64) * t15298 + F::cast_from(0.14671e0_f64) * t15302 - F::cast_from(0.14671e0_f64) * t15304 - F::cast_from(0.17116166666666666667e0_f64) * t15306 - F::cast_from(0.36793333333333333333e0_f64) * t15308;
    let t15312 = t891 * t15310 * t898;
    let t15316 = F::cast_from(1.0_f64) / t2977 / t85;
    (t15312, t15316)
}
