//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 749/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk749<F: Float>(t15316: F, t73: F, t2950: F, t879: F, t2981: F, t15292: F, t15294: F, t15296: F, t15298: F, t15302: F, t15304: F, t15306: F, t15308: F) -> (F, F, F, F) {
    let t15317 = t73 * t15316;
    let t15318 = t2950 * t879;
    let t15319 = t15318 * t2981;
    let t15330 = -F::new(0.50638e1) * t15292 + F::new(0.16879333333333333333e1) * t15294 - F::new(0.19692555555555555555e1) * t15296 - F::new(0.93011851851851851854e0) * t15298 + F::new(0.27303333333333333333e0) * t15302 - F::new(0.27303333333333333333e0) * t15304 - F::new(0.3185388888888888889e0) * t15306 - F::new(0.36514074074074074075e0) * t15308;
    (t15317, t15318, t15319, t15330)
}
