//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1130/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1130(t140: f64, t4227: f64, t1098: f64, t1015: f64, t4246: f64, t3068: f64, t1562: f64, t2850: f64, t2846: f64, t1554: f64, t3025: f64, t1125: f64, t12483: f64, t12487: f64, t12492: f64, t12498: f64, t12503: f64, t12507: f64, t12512: f64, t12516: f64, t12520: f64, t12524: f64, t12530: f64, t3035: f64, t3040: f64, t3044: f64, t3052: f64, t3067: f64, t3099: f64, t4212: f64, t4265: f64, t9618: f64, t9626: f64) -> f64 {
    let t12535 = t140 * t4227;
    let t12537 = t1098 * t12535 / 432.0_f64;
    let t12538 = t4246 * t1015;
    let t12539 = t3068 * t12538;
    let t12542 = t1562 * t2850;
    let t12543 = t3068 * t12542;
    let t12546 = t1562 * t2846;
    let t12547 = t3068 * t12546;
    let t12550 = t1554 * t3025;
    let t12552 = 5.0_f64 / 6912.0_f64 * t1125 * t12483 + 5.0_f64 / 13824.0_f64 * t1125 * t12487 + 5.0_f64 / 2304.0_f64 * t1125 * t12492 - 5.0_f64 / 2592.0_f64 * t4265 * t3099 + t3052 * t12498 / 1536.0_f64 + t9618 * t12503 / 512.0_f64 - t9626 * t12507 / 512.0_f64 - 5.0_f64 / 5184.0_f64 * t1125 * t12512 - t1125 * t12516 / 1152.0_f64 - t1125 * t12520 / 2304.0_f64 - t1098 * t12524 / 288.0_f64 - t4212 * t3035 / 81.0_f64 + t12530 + t4212 * t3044 / 108.0_f64 + t4212 * t3040 / 54.0_f64 - t12537 - t3067 * t12539 / 2304.0_f64 - t3067 * t12543 / 4608.0_f64 - t3067 * t12547 / 2304.0_f64 + t12550 / 162.0_f64;
    t12552
}
