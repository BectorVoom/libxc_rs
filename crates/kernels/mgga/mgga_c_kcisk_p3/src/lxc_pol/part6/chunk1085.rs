//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1085/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1085<F: Float>(t10328: F, t15799: F, t15800: F, t15803: F, t15804: F, t30145: F, t31807: F, t5581: F, t7725: F, t7730: F, t7733: F, t8: F, t8461: F, t8465: F, t8467: F, t8469: F, t8474: F, t8477: F, t9297: F) -> F {
    let tv3rho33 = t8 * (t30145 + t31807) - t10328 + t15799 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t7730 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t7733 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t8461 + t15800 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8465 + F::cast_from(3.0_f64) * t7725 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t9297 - t15803 + t15804 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t8474 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8477 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8467 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t8469 + F::cast_from(6.0_f64) * t5581;
    tv3rho33
}
