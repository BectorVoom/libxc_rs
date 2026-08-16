//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1167/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1167(t1238: f64, t15820: f64, t1761: f64, t18287: f64, t19121: f64, t19209: f64, t19211: f64, t19214: f64, t19220: f64, t19226: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5060: f64, t6268: f64) -> f64 {
    let t19231 = -t1238 * t19209 + 4.0_f64 * t1238 * t19214 + 2.0_f64 * t1238 * t19220 - 6.0_f64 * t1238 * t19226 - 2.0_f64 * t15820 * t1761 + t18287 * t498 + t19121 * t498 + t19211 * t498 - t3487 * t6268 - t3593 * t6268 + 4.0_f64 * t4945 * t5060 + 4.0_f64 * t5055 * t5060;
    t19231
}
