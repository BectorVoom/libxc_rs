//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1033/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1033(t225: f64, t5848: f64, t68: f64, t369: f64, t1539: f64, t1616: f64, t3071: f64, t1020: f64, t1041: f64, t1618: f64, t1622: f64, t3039: f64, t3070: f64, t3084: f64, t3130: f64, t3160: f64, t378: f64, t4572: f64, t4604: f64, t4625: f64, t4631: f64, t4641: f64, t4644: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t5880: f64, t5885: f64, t5890: f64, t5894: f64, t5900: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5903 = t5848 * t225;
    let t5904 = t5903 * t68;
    let t5905 = t5904 * t369;
    let t5908 = t1616 * t1539;
    let t5909 = t3071 * t5908;
    let t5914 = t1041 * t5857 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t1041 * t5861 + t4644 * t1622 / 2304.0_f64 + t1020 * t5869 / 3072.0_f64 + t3130 * t5875 / 1536.0_f64 - t3039 * t5880 / 3072.0_f64 - t3160 + t4625 / 2304.0_f64 - t973 * t5885 / 144.0_f64 + t4604 / 432.0_f64 + t973 * t5890 / 288.0_f64 + t973 * t5894 / 216.0_f64 + t4572 / 3456.0_f64 + t4631 / 2304.0_f64 - t1041 * t5900 / 2304.0_f64 - t3084 + t5905 * t378 / 3072.0_f64 + t3070 * t5909 / 2304.0_f64 + t4641 * t1618 / 1536.0_f64;
    (t5903, t5904, t5905, t5908, t5909, t5914)
}
