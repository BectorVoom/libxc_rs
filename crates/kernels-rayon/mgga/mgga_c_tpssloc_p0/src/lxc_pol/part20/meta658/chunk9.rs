//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2449/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449(t10250: f64, t1041: f64, t10884: f64, t14172: f64, t14184: f64, t1607: f64, t1616: f64, t1618: f64, t3048: f64, t3070: f64, t3071: f64, t3117: f64, t42358: f64, t42554: f64, t42756: f64, t43167: f64, t4582: f64, t4593: f64, t48554: f64, t50078: f64, t50084: f64, t50094: f64, t50098: f64, t50100: f64) -> f64 {
    let t50102 = 5.0_f64 / 4608.0_f64 * t3117 * t14184 - 5.0_f64 / 768.0_f64 * t1041 * t4582 * t14172 * t48554 + t42756 * t1618 / 3072.0_f64 + t50078 - t3070 * t3071 * t1616 * t10250 / 768.0_f64 - t50084 / 1152.0_f64 + t43167 / 768.0_f64 - t42358 * t4582 * t4593 * t10884 / 3072.0_f64 - 5.0_f64 / 864.0_f64 * t3048 * t14184 + t50094 / 1152.0_f64 - 77.0_f64 / 486.0_f64 * t42554 * t1607 + 11.0_f64 / 324.0_f64 * t50098 + t50100 / 144.0_f64;
    t50102
}
