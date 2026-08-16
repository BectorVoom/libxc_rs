//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1628/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1628(t17712: f64, t4594: f64, t4582: f64, t1023: f64, t1041: f64, t10413: f64, t10436: f64, t10511: f64, t10871: f64, t14049: f64, t14059: f64, t17688: f64, t17693: f64, t17697: f64, t17701: f64, t17705: f64, t3039: f64, t3070: f64, t3114: f64, t3130: f64, t4585: f64, t4590: f64, t4644: f64, t5869: f64) -> (f64, f64, f64, f64, f64) {
    let t17713 = t17712 * t4594;
    let t17714 = t4582 * t17713;
    let t17717 = t17712 * t1023;
    let t17718 = t4582 * t17717;
    let t17725 = -t14049 - t10436 / 13824.0_f64 - 5.0_f64 / 2304.0_f64 * t1041 * t17688 + 5.0_f64 / 6912.0_f64 * t1041 * t17693 + 5.0_f64 / 5184.0_f64 * t1041 * t17697 - t10413 * t17701 / 4608.0_f64 + t3070 * t17705 / 2304.0_f64 - t4644 * t4585 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t4644 * t4590 - t14059 + t3130 * t17714 / 1536.0_f64 - t3039 * t17718 / 3072.0_f64 + t3114 * t5869 / 3072.0_f64 - t10511 / 13824.0_f64 - t10871 / 20736.0_f64;
    (t17713, t17714, t17717, t17718, t17725)
}
