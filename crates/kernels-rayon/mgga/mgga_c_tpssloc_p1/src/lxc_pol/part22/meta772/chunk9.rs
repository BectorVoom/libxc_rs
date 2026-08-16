//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2641/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2641(t22334: f64, t225: f64, t1238: f64, t1251: f64, t1252: f64, t15797: f64, t15820: f64, t1761: f64, t19209: f64, t19214: f64, t19220: f64, t19232: f64, t19234: f64, t22007: f64, t22008: f64, t3593: f64, t3598: f64, t45350: f64, t4945: f64, t5055: f64, t5060: f64, t5088: f64, t5089: f64, t6244: f64, t6267: f64, t6268: f64, t66822: f64) -> f64 {
    let t73856 = t22334 * t225;
    let t73885 = 24.0_f64 * t1238 * t1251 * t22007 * t45350 + 6.0_f64 * t1238 * t3598 * t5088 * t6267 - 3.0_f64 * t1252 * t73856 - 3.0_f64 * t15797 * t6268 + 6.0_f64 * t15820 * t6244 - 3.0_f64 * t15820 * t6268 - 6.0_f64 * t1761 * t66822 - 3.0_f64 * t19209 * t5055 + 12.0_f64 * t19214 * t4945 + 6.0_f64 * t19220 * t4945 - 3.0_f64 * t19232 * t5089 + 12.0_f64 * t19234 * t5060 - 6.0_f64 * t22008 * t3593;
    t73885
}
