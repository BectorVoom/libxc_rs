//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2429/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429(t13748: f64, t2960: f64, t1025: f64, t10884: f64, t10937: f64, t14041: f64, t1539: f64, t2780: f64, t3070: f64, t3071: f64, t42483: f64, t42552: f64, t42557: f64, t42578: f64, t42582: f64, t4650: f64, t49658: f64, t49662: f64, t49666: f64, t49678: f64, t49682: f64) -> f64 {
    let t49684 = t2960 * t13748;
    let t49688 = -2.0_f64 / 81.0_f64 * t49658 - t49662 - t10937 * t14041 / 288.0_f64 + t49666 / 2304.0_f64 + t42483 * t3071 * t1539 * t10884 / 4608.0_f64 + t3070 * t3071 * t4650 * t2780 / 1536.0_f64 + 5.0_f64 / 1296.0_f64 * t42552 + 11.0_f64 / 324.0_f64 * t42557 + 19.0_f64 / 576.0_f64 * t49678 * t1025 + t49682 / 1152.0_f64 + t49684 / 27.0_f64 - t42578 / 144.0_f64 - t42582 / 144.0_f64;
    t49688
}
