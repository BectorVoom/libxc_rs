//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2369/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369(t1041: f64, t13969: f64, t14142: f64, t14179: f64, t10309: f64, t10408: f64, t14126: f64, t14167: f64, t1616: f64, t2776: f64, t3070: f64, t3071: f64, t3117: f64, t42478: f64, t42481: f64, t42490: f64, t42546: f64, t43358: f64, t4579: f64, t4582: f64, t4650: f64, t47779: f64, t47915: f64, t48260: f64, t48497: f64, t48607: f64) -> f64 {
    let t48626 = t1041 * t13969 * t14142;
    let t48629 = t1041 * t13969 * t14179;
    let t48656 = t3117 * t14167 / 256.0_f64 - t48626 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t48629 + 5.0_f64 / 384.0_f64 * t1041 * t4582 * t47779 * t48497 - t3070 * t3071 * t4650 * t2776 / 768.0_f64 + t48607 * t3071 * t47915 / 256.0_f64 - 5.0_f64 / 768.0_f64 * t48607 * t10408 * t48260 - 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t1616 * t10309 + 19.0_f64 / 864.0_f64 * t43358 * t4579 - t42478 / 2304.0_f64 + t42481 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t42490 - t42546 * t14126 / 1536.0_f64;
    t48656
}
