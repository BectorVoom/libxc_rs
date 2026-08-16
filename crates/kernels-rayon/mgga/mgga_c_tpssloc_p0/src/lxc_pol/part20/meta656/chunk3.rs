//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2427/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427(t1615: f64, t3120: f64, t3040: f64, t10403: f64, t10422: f64, t14214: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t1022: f64, t10408: f64, t10413: f64, t10937: f64, t14174: f64, t14207: f64, t14211: f64, t14212: f64, t14220: f64, t14222: f64, t14235: f64, t14491: f64, t2244: f64, t2250: f64, t2770: f64, t3071: f64, t3114: f64, t3117: f64, t3123: f64, t3134: f64, t42483: f64, t42508: f64, t42530: f64, t4337: f64, t49594: f64) -> (f64, f64, f64, f64) {
    let t49616 = t1615 * t3120;
    let t49621 = t1615 * t3040;
    let t49629 = t10403 * t10422 * t14214;
    let t49649 = t4552 * t3030;
    let t49650 = t49649 * t3032;
    let t49651 = t49650 * t3129;
    let t49654 = 5.0_f64 / 2304.0_f64 * t10403 * t10408 * t14211 * t1022 * t2770 * t2244 - t10413 * t3071 * t49616 * t14220 / 1536.0_f64 + t42483 * t3071 * t49621 * t14220 / 1536.0_f64 - 5.0_f64 / 432.0_f64 * t10937 * t14235 + t49629 / 576.0_f64 + t42508 * t14222 / 144.0_f64 + t10403 * t3071 * t14211 * t14212 * t2250 / 768.0_f64 + t42530 / 864.0_f64 - 5.0_f64 / 4608.0_f64 * t10413 * t10408 * t4337 * t49594 + t3114 * t14491 / 1024.0_f64 - 5.0_f64 / 768.0_f64 * t3117 * t14174 + t14207 * t3123 / 1024.0_f64 + t49651 * t3134 / 512.0_f64;
    (t49616, t49649, t49650, t49654)
}
