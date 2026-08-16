//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1062/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1062<F: Float>(t3180: F, t3463: F, t3275: F, t3188: F, t10328: F, t12816: F, t15796: F, t3300: F, t3301: F, t3466: F, t3468: F, t3470: F, t3475: F, t3478: F, t4571: F, t4576: F, t4579: F, t5558: F, t8: F) -> F {
    let t15799 = F::cast_from(3.0_f64) * t3180;
    let t15800 = F::cast_from(3.0_f64) * t3463;
    let t15803 = F::cast_from(3.0_f64) * t3275;
    let t15804 = F::cast_from(6.0_f64) * t3188;
    let tv3rho30 = -F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t5558 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t4576 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t4579 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3466 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t4571 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3475 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3478 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3468 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3470 - t10328 + t8 * (t12816 + t15796) + t15799 + t15800 + F::cast_from(3.0_f64) * t3300 + F::cast_from(6.0_f64) * t3301 - t15803 + t15804;
    tv3rho30
}
