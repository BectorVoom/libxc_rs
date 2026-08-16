//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3210/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210<F: Float>(t19121: F, t225: F, t19259: F, t11613: F, t1252: F, t14972: F, t15425: F, t15787: F, t15794: F, t15797: F, t1751: F, t1761: F, t19209: F, t19220: F, t19232: F, t19234: F, t19249: F, t3481: F, t3487: F, t3600: F, t3631: F, t4940: F, t4945: F, t498: F, t5052: F, t5055: F, t5060: F, t5089: F, t53658: F, t6238: F, t6268: F) -> F {
    let t66845 = t19121 * t225;
    let t66860 = t19259 * t225;
    let t66879 = F::cast_from(2.0_f64) * t15425 * t1751 * t498 + t3481 * t498 * t6238 + F::cast_from(4.0_f64) * t4940 * t498 * t5052 - F::cast_from(2.0_f64) * t11613 * t6268 - F::cast_from(2.0_f64) * t1252 * t66845 - F::cast_from(2.0_f64) * t1252 * t66860 + F::cast_from(8.0_f64) * t14972 * t5060 - F::cast_from(4.0_f64) * t14972 * t5089 - F::cast_from(2.0_f64) * t15787 * t5055 - F::cast_from(12.0_f64) * t15794 * t4945 + F::cast_from(8.0_f64) * t15797 * t5060 - F::cast_from(2.0_f64) * t1761 * t53658 - F::cast_from(2.0_f64) * t19209 * t3487 + F::cast_from(4.0_f64) * t19220 * t3487 - t19232 * t3631 + F::cast_from(4.0_f64) * t19234 * t3600 - t19249 * t3631;
    t66879
}
