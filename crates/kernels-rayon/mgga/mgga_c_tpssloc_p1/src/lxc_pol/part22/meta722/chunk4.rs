//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2360/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360(t1530: f64, t16596: f64, t16944: f64, t17120: f64, t1877: f64, t2522: f64, t41258: f64, t41262: f64, t4310: f64, t4314: f64, t46436: f64, t59584: f64, t67487: f64, t67488: f64, t67489: f64, t67490: f64, t67494: f64) -> f64 {
    let t68391 = -3.0_f64 * t1530 * t1877 * t59584 + 18.0_f64 * t16596 * t17120 * t2522 + 36.0_f64 * t16944 * t4310 * t4314 - t41258 - t41262 + t46436 - t67487 + t67488 - t67489 - t67490 + t67494;
    t68391
}
