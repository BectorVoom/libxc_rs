//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2460/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460(t10305: f64, t10390: f64, t10857: f64, t10858: f64, t10891: f64, t14041: f64, t14103: f64, t1539: f64, t1616: f64, t3070: f64, t3071: f64, t3121: f64, t3130: f64, t3131: f64, t42397: f64, t43325: f64, t43332: f64, t43336: f64, t43341: f64, t43350: f64, t43352: f64, t43354: f64, t4347: f64, t4582: f64, t4593: f64) -> f64 {
    let t50423 = t43325 / 81.0_f64 + 5.0_f64 / 5184.0_f64 * t3070 * t42397 * t1616 * t10305 + t10390 * t14041 / 1536.0_f64 + t3070 * t3071 * t4347 * t3121 / 1536.0_f64 + t3070 * t3071 * t1539 * t10858 / 4608.0_f64 + t43332 / 216.0_f64 + t43336 / 3456.0_f64 - 5.0_f64 / 20736.0_f64 * t43341 + t3130 * t4582 * t4593 * t3131 * t10857 / 1536.0_f64 + t10891 * t14103 / 192.0_f64 + t43350 / 1536.0_f64 - t43352 / 4608.0_f64 - 19.0_f64 / 2592.0_f64 * t43354;
    t50423
}
