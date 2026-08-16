//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2269/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2269(t12568: f64, t608: f64, t2251: f64, t3953: f64, t1437: f64, t2303: f64, t72: f64, t1865: f64, t22523: f64, t22554: f64, t26055: f64, t26063: f64, t26067: f64, t6490: f64, t6506: f64, t6510: f64, t7432: f64, t83750: f64, t83760: f64, t83775: f64) -> f64 {
    let t90202 = t12568 * t608;
    let t90205 = t3953 * t2251;
    let t90227 = t72 * t2303 * t1437;
    let t90230 = 2.0_f64 / 3.0_f64 * t90202 * t1865 + t90205 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26055 * t6506 + 2.0_f64 / 3.0_f64 * t26055 * t6510 + 5.0_f64 / 6.0_f64 * t83775 * t7432 + 5.0_f64 / 3.0_f64 * t83750 * t7432 + 5.0_f64 / 3.0_f64 * t22554 * t26063 + 5.0_f64 / 3.0_f64 * t22554 * t26067 + 5.0_f64 / 6.0_f64 * t83760 * t7432 + 5.0_f64 / 3.0_f64 * t22523 * t26063 + 5.0_f64 / 3.0_f64 * t22523 * t26067 + 5.0_f64 / 6.0_f64 * t6490 * t90227;
    t90230
}
