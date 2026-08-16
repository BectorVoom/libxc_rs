//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2207/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207(t12757: f64, t12808: f64, t1453: f64, t2331: f64, t2358: f64, t26129: f64, t29903: f64, t45424: f64, t45428: f64, t45430: f64, t45435: f64, t45676: f64, t45689: f64, t45690: f64, t45731: f64, t45775: f64, t64: f64, t656: f64, t666: f64, t9366: f64) -> f64 {
    let t45780 = 3.0_f64 / 4.0_f64 * t64 * t2331 * t12808 * t666 + 3.0_f64 / 4.0_f64 * t64 * t12757 * t2358 + 6.0_f64 * t45676 + 3.0_f64 * t64 * t45435 * t1453 * t9366 - 9.0_f64 / 4.0_f64 * t29903 * t26129 * t2358 + 22.0_f64 / 3.0_f64 * t45424 + 2.0_f64 * t45428 - 2.0_f64 * t45430 - t45689 + t45690 - t64 * t656 * (t45731 + t45775) / 8.0_f64;
    t45780
}
