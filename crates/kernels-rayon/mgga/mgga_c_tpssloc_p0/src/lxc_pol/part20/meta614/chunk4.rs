//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2208/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208(t109: f64, t2332: f64, t4043: f64, t4067: f64, t45421: f64, t45422: f64, t45426: f64, t45432: f64, t45656: f64, t45659: f64, t45660: f64, t45662: f64, t45780: f64, t64: f64, t9365: f64, t9411: f64) -> f64 {
    let t110 = 1.0_f64 < t109;
    let t45782 = piecewise3(t110, 0.0_f64, t45421 + 154.0_f64 / 9.0_f64 * t45422 - 11.0_f64 / 3.0_f64 * t45426 + t45432 / 3.0_f64 + t64 * t4043 * t9411 / 4.0_f64 + 154.0_f64 / 27.0_f64 * t45656 + t45659 - 4.0_f64 * t45660 - 2.0_f64 * t45662 - 9.0_f64 / 4.0_f64 * t64 * t9365 * t4067 * t2332 + t45780);
    t45782
}
