//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 495/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk495<F: Float>(t6163: F, t6257: F, t1168: F, t1174: F, t1452: F, t1454: F, t1455: F, t1459: F, t1857: F, t1860: F, t228: F, t4444: F, t458: F, t462: F, t5555: F, t5558: F, t598: F, t6071: F, t6073: F, t6077: F, t6080: F, t6086: F, t6093: F, t6096: F, t6099: F, t6102: F, t6105: F) -> (F, F) {
    let t6258 = t6163 + t6257;
    let t6261 = t6071 * t228 + t6073 * t1455 + t1857 * t1168 / F::cast_from(4.0_f64) + F::cast_from(2.0_f64) * t598 * t6077 + t6080 * t1455 + t1860 * t1168 / F::cast_from(4.0_f64) + t1452 * t1459 / F::cast_from(2.0_f64) + t1454 * t6086 / F::cast_from(2.0_f64) - F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t598 * t5555 + t598 * t5558 / F::cast_from(2.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t458 * t6093 + F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t4444 * t6096 - F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t1174 * t6099 + t458 * t6102 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t1174 * t6105 + t462 * t6258 / F::cast_from(4.0_f64);
    (t6258, t6261)
}
