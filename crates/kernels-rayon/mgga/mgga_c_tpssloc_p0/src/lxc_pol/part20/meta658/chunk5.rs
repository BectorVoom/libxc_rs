//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2445/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2445(t10390: f64, t10860: f64, t13536: f64, t14235: f64, t1622: f64, t3070: f64, t3073: f64, t42397: f64, t42648: f64, t43114: f64, t43118: f64, t43298: f64, t4641: f64, t49964: f64, t49966: f64, t49972: f64, t49976: f64, t49984: f64, t49987: f64, t49989: f64) -> f64 {
    let t49991 = -t43298 * t1622 / 288.0_f64 + t49964 / 768.0_f64 + t49966 / 1152.0_f64 + 19.0_f64 / 864.0_f64 * t42648 * t1622 + t4641 * t10860 / 3072.0_f64 - t49972 / 216.0_f64 - t43114 / 3456.0_f64 + 5.0_f64 / 1728.0_f64 * t3070 * t42397 * t13536 * t49976 + 5.0_f64 / 2304.0_f64 * t10390 * t14235 + t43118 / 4608.0_f64 - t49984 * t3073 / 144.0_f64 - t49987 / 144.0_f64 - t49989 / 144.0_f64;
    t49991
}
