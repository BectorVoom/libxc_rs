//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2593/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593(t11728: f64, t22312: f64, t248: f64, t3570: f64, t1174: f64, t1177: f64, t15495: f64, t6221: f64, t65552: f64, t65554: f64, t65558: f64, t65567: f64, t71189: f64, t71201: f64, t72273: f64, t72285: f64, t72287: f64, t72289: f64, t72293: f64) -> f64 {
    let t72297 = t11728 * t248 * t3570 * t22312;
    let t72299 = t65552 / 3456.0_f64 + t65554 / 1536.0_f64 - t72273 / 6912.0_f64 - t65558 / 2304.0_f64 + t65567 / 36.0_f64 - t1174 * t1177 * t71201 / 48.0_f64 - t1174 * t1177 * t71189 / 48.0_f64 - t15495 * t6221 / 192.0_f64 - t72285 / 1152.0_f64 + t72287 / 768.0_f64 + t72289 / 432.0_f64 + t72293 / 4608.0_f64 - t72297 / 768.0_f64;
    t72299
}
