//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 907/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk907<F: Float>(t1216: F, t313: F, t6678: F, t806: F, t810: F, t8316: F, t8323: F, t8326: F, t8329: F, t8337: F, t8344: F, t8347: F, t8350: F, t8377: F, t8385: F) -> F {
    let t8395 = F::cast_from(3.0_f64) / F::cast_from(10.0_f64) * t313 * (-F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t8316 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t8377 * t1216 * t806 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t8323 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t8326 - F::cast_from(5.0_f64) * t8329 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t8337 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t8385 * t1216 * t810 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t8344 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t8347 + F::cast_from(5.0_f64) * t8350) - t6678;
    t8395
}
