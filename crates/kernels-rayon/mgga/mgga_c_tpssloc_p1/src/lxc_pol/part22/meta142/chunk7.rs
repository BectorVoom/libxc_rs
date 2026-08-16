//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 922/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk922(t1484: f64, t2523: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2522: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64, t4209: f64, t4213: f64, t4214: f64, t4215: f64, t4216: f64) -> f64 {
    let t4320 = t2523 * t1484;
    let t4323 = 3.0_f64 * t2522 * t4320 + t2408 + t2417 - t2423 - t2426 - t2486 + t2518 - t2530 - t2537 + t2538 + t2665 + t4209 - t4213 + t4214 - t4215 - t4216;
    t4323
}
