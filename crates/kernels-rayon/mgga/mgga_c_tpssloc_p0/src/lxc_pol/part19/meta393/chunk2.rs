//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1493/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493(t2331: f64, t2332: f64, t2358: f64, t45421: f64, t45422: f64, t45424: f64, t45426: f64, t45428: f64, t45430: f64, t45432: f64, t45435: f64, t45436: f64, t45444: f64, t45505: f64, t64: f64, t656: f64, t9365: f64, t9370: f64, t9411: f64) -> f64 {
    let t45509 = t45421 + 616.0_f64 / 27.0_f64 * t45422 + 44.0_f64 / 3.0_f64 * t45424 - 22.0_f64 / 3.0_f64 * t45426 + 8.0_f64 * t45428 - 8.0_f64 * t45430 + 4.0_f64 / 3.0_f64 * t45432 + 3.0_f64 * t64 * t45435 * t45436 - 9.0_f64 / 2.0_f64 * t64 * t9365 * t2332 * t2358 + 3.0_f64 / 4.0_f64 * t64 * t2331 * t45444 + t64 * t9370 * t9411 - t64 * t656 * t45505 / 8.0_f64;
    t45509
}
