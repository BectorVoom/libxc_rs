//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1269/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1269(t1078: f64, t1982: f64, t93488: f64, t25604: f64, t25610: f64, t3093: f64, t4975: f64, t3058: f64, t8521: f64, t7135: f64, t989: f64, t1000: f64, t1071: f64, t1096: f64, t1097: f64, t11247: f64, t12052: f64, t12178: f64, t1986: f64, t25586: f64, t25588: f64, t25596: f64, t25612: f64, t25625: f64, t25640: f64, t25651: f64, t25674: f64, t25683: f64, t25695: f64, t3042: f64, t3067: f64, t7144: f64, t7145: f64, t7153: f64, t7156: f64, t7159: f64, t7160: f64, t7168: f64, t7170: f64, t93438: f64, t93459: f64, t93464: f64, t93471: f64, t93485: f64) -> (f64, f64) {
    let t93490 = t1982 * t93488 * t1078;
    let t93497 = t25610 * t25604;
    let t93498 = t3093 * t4975;
    let t93502 = t3058 * t8521;
    let t93509 = t989 * t7135;
    let t93514 = -0.39512695097613069591e1_f64 * t93459 * t1000 - 0.13010442282307799193e1_f64 * t93464 * t7170 - 0.39512695097613069591e1_f64 * t25651 * t12178 - 0.4336814094102599731e0_f64 * t93471 * t7168 * t11247 * t12052 - 0.13010442282307799193e1_f64 * t7156 * t25588 + 0.26020884564615598386e1_f64 * t7159 * t7160 * t25586 * t1096 - 0.26020884564615598386e1_f64 * t25640 * t25683 + 0.26020884564615598386e1_f64 * t93485 * t7153 - 0.26020884564615598386e1_f64 * t93490 * t25674 - 0.26020884564615598386e1_f64 * t7144 * t7145 * t7135 * t3042 - 0.10408353825846239354e2_f64 * t93497 * t25596 * t93498 + 0.10408353825846239354e2_f64 * t93502 * t25612 * t93438 - 0.26020884564615598386e1_f64 * t25625 * t1071 * t1986 - 0.39512695097613069591e1_f64 * t93509 * t1097 + 0.39512695097613069591e1_f64 * t25695 * t3067;
    (t93490, t93514)
}
