//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2984/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984(t1036: f64, t17878: f64, t13969: f64, t17631: f64, t3039: f64, t3082: f64, t5905: f64, t10937: f64, t10952: f64, t17632: f64, t17677: f64, t17960: f64, t2986: f64, t3070: f64, t3071: f64, t43110: f64, t48585: f64, t49889: f64, t49892: f64, t49894: f64, t49897: f64, t49906: f64, t49922: f64, t50370: f64, t55716: f64, t884: f64) -> f64 {
    let t62343 = t17878 * t1036;
    let t62349 = t3039 * t13969 * t17631;
    let t62360 = t5905 * t3082;
    let t62362 = t3070 * t3071 * t17960 * t884 / 2304.0_f64 + t49889 / 162.0_f64 - 5.0_f64 / 972.0_f64 * t49892 - t49894 / 1152.0_f64 - t49897 / 1152.0_f64 + t49906 / 162.0_f64 + t62343 / 2304.0_f64 + t43110 / 648.0_f64 - t10937 * t17677 / 216.0_f64 - t62349 / 1152.0_f64 - t10952 * t17632 / 768.0_f64 - t49922 / 1728.0_f64 - t2986 * t50370 * t55716 / 9.0_f64 + 7.0_f64 / 162.0_f64 * t2986 * t48585 * t55716 - t62360 / 13824.0_f64;
    t62362
}
