//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3210/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210(t19121: f64, t225: f64, t19259: f64, t11613: f64, t1252: f64, t14972: f64, t15425: f64, t15787: f64, t15794: f64, t15797: f64, t1751: f64, t1761: f64, t19209: f64, t19220: f64, t19232: f64, t19234: f64, t19249: f64, t3481: f64, t3487: f64, t3600: f64, t3631: f64, t4940: f64, t4945: f64, t498: f64, t5052: f64, t5055: f64, t5060: f64, t5089: f64, t53658: f64, t6238: f64, t6268: f64) -> f64 {
    let t66845 = t19121 * t225;
    let t66860 = t19259 * t225;
    let t66879 = 2.0_f64 * t15425 * t1751 * t498 + t3481 * t498 * t6238 + 4.0_f64 * t4940 * t498 * t5052 - 2.0_f64 * t11613 * t6268 - 2.0_f64 * t1252 * t66845 - 2.0_f64 * t1252 * t66860 + 8.0_f64 * t14972 * t5060 - 4.0_f64 * t14972 * t5089 - 2.0_f64 * t15787 * t5055 - 12.0_f64 * t15794 * t4945 + 8.0_f64 * t15797 * t5060 - 2.0_f64 * t1761 * t53658 - 2.0_f64 * t19209 * t3487 + 4.0_f64 * t19220 * t3487 - t19232 * t3631 + 4.0_f64 * t19234 * t3600 - t19249 * t3631;
    t66879
}
