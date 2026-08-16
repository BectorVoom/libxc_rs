//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2541/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541(t11190: f64, t15060: f64, t3265: f64, t11129: f64, t11306: f64, t11307: f64, t11310: f64, t11350: f64, t11361: f64, t11415: f64, t11420: f64, t11421: f64, t15146: f64, t15210: f64, t15226: f64, t15229: f64, t1683: f64, t3333: f64, t3357: f64, t44220: f64, t4820: f64, t4823: f64, t4861: f64, t51427: f64, t51437: f64, t51439: f64, t51441: f64, t51443: f64, t51446: f64) -> (f64, f64) {
    let t51449 = 0.28947563097646563121e3_f64 * t11190 * t15060 * t3265;
    let t51450 = 0.30762056574649219973e4_f64 * t44220 * t15226 + 0.10526802520742363173e2_f64 * t11361 * t15210 + 18.0_f64 * t11415 * t15229 + 0.11579025239058625248e4_f64 * t11350 * t4823 * t11306 + 18.0_f64 * t3357 * t4820 * t3333 + 6.0_f64 * t15146 * t11307 - 0.19298375398431042081e3_f64 * t51427 * t11421 - 24.0_f64 * t11420 * t1683 * t11306 + 0.6233709278045326953e3_f64 * t11310 * t4861 * t11129 + t51437 + t51439 - t51441 - t51443 + t51446 + t51449;
    (t51449, t51450)
}
