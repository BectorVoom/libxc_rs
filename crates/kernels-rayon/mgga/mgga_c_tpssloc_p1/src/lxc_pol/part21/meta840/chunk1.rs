//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3014/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3014(t1022: f64, t1058: f64, t1060: f64, t11034: f64, t11046: f64, t11054: f64, t11059: f64, t11065: f64, t11066: f64, t14577: f64, t14587: f64, t14630: f64, t14651: f64, t18047: f64, t18080: f64, t18086: f64, t18093: f64, t18107: f64, t18121: f64, t18162: f64, t3120: f64, t3180: f64, t3186: f64, t3193: f64, t3200: f64, t43480: f64, t4669: f64, t4677: f64, t4681: f64, t5928: f64, t5929: f64, t5932: f64, t5936: f64) -> f64 {
    let t63095 = 2.0_f64 * t18086 * t3193 + 12.0_f64 * t11059 * t5932 * t14577 + 4.0_f64 * t11034 * t18121 + 2.0_f64 * t1058 * t18047 * t1022 * t1060 + 2.0_f64 * t43480 * t5929 + 4.0_f64 * t14651 * t4681 - 4.0_f64 * t3200 * t4677 * t18107 - 6.0_f64 * t11065 * t5928 * t11066 * t3120 + 2.0_f64 * t3180 * t18162 + 4.0_f64 * t4669 * t14587 + 2.0_f64 * t3186 * t5936 * t11054 + 2.0_f64 * t11046 * t18080 * t18093 + t11046 * t5936 * t14630;
    t63095
}
