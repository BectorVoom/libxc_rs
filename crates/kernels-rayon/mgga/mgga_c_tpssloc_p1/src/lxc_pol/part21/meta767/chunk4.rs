//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2649/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649(t4067: f64, t2331: f64, t45421: f64, t45422: f64, t45424: f64, t45426: f64, t45656: f64, t45658: f64, t45660: f64, t45662: f64, t45688: f64, t45690: f64, t55420: f64, t55457: f64, t55512: f64, t64: f64, t656: f64) -> f64 {
    let t55517 = t4067 * t4067;
    let t55530 = 2.0_f64 / 3.0_f64 * t55420 - t64 * t656 * (t55457 + t55512) / 8.0_f64 + t64 * t2331 * t55517 / 2.0_f64 + t45421 + 308.0_f64 / 27.0_f64 * t45656 + 88.0_f64 / 9.0_f64 * t45658 - 8.0_f64 / 3.0_f64 * t45660 - 4.0_f64 / 3.0_f64 * t45662 + 308.0_f64 / 27.0_f64 * t45422 + 22.0_f64 / 9.0_f64 * t45424 - 11.0_f64 / 9.0_f64 * t45426 - 44.0_f64 / 9.0_f64 * t45688 + 2.0_f64 / 3.0_f64 * t45690;
    t55530
}
