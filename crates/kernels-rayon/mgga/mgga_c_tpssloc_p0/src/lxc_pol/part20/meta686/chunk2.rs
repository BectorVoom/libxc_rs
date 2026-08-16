//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2601/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601(t1174: f64, t14753: f64, t3431: f64, t14744: f64, t11651: f64, t15438: f64, t1227: f64, t13969: f64, t15540: f64, t15530: f64, t3515: f64, t11638: f64, t11688: f64, t15740: f64, t3506: f64, t3508: f64, t44621: f64, t44886: f64, t44890: f64, t44894: f64, t4582: f64, t4977: f64, t50924: f64) -> f64 {
    let t52773 = t1174 * t3431 * t14753;
    let t52776 = t1174 * t3431 * t14744;
    let t52781 = t15438 * t11651;
    let t52792 = t1227 * t13969 * t15540;
    let t52795 = t3515 * t13969 * t15530;
    let t52797 = -t15740 * t11688 / 768.0_f64 - t52773 / 144.0_f64 - t52776 / 48.0_f64 + 35.0_f64 / 972.0_f64 * t1174 * t44621 * t50924 - t52781 / 1536.0_f64 + t3506 * t4582 * t4977 * t3508 * t11638 / 1536.0_f64 - t44886 / 4608.0_f64 - t44890 / 2304.0_f64 + t44894 / 4608.0_f64 + 5.0_f64 / 3456.0_f64 * t52792 - t52795 / 1536.0_f64;
    t52797
}
