//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1269/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1269<F: Float>(t1078: F, t1982: F, t93488: F, t25604: F, t25610: F, t3093: F, t4975: F, t3058: F, t8521: F, t7135: F, t989: F, t1000: F, t1071: F, t1096: F, t1097: F, t11247: F, t12052: F, t12178: F, t1986: F, t25586: F, t25588: F, t25596: F, t25612: F, t25625: F, t25640: F, t25651: F, t25674: F, t25683: F, t25695: F, t3042: F, t3067: F, t7144: F, t7145: F, t7153: F, t7156: F, t7159: F, t7160: F, t7168: F, t7170: F, t93438: F, t93459: F, t93464: F, t93471: F, t93485: F) -> (F, F) {
    let t93490 = t1982 * t93488 * t1078;
    let t93497 = t25610 * t25604;
    let t93498 = t3093 * t4975;
    let t93502 = t3058 * t8521;
    let t93509 = t989 * t7135;
    let t93514 = -F::cast_from(0.39512695097613069591e1_f64) * t93459 * t1000 - F::cast_from(0.13010442282307799193e1_f64) * t93464 * t7170 - F::cast_from(0.39512695097613069591e1_f64) * t25651 * t12178 - F::cast_from(0.4336814094102599731e0_f64) * t93471 * t7168 * t11247 * t12052 - F::cast_from(0.13010442282307799193e1_f64) * t7156 * t25588 + F::cast_from(0.26020884564615598386e1_f64) * t7159 * t7160 * t25586 * t1096 - F::cast_from(0.26020884564615598386e1_f64) * t25640 * t25683 + F::cast_from(0.26020884564615598386e1_f64) * t93485 * t7153 - F::cast_from(0.26020884564615598386e1_f64) * t93490 * t25674 - F::cast_from(0.26020884564615598386e1_f64) * t7144 * t7145 * t7135 * t3042 - F::cast_from(0.10408353825846239354e2_f64) * t93497 * t25596 * t93498 + F::cast_from(0.10408353825846239354e2_f64) * t93502 * t25612 * t93438 - F::cast_from(0.26020884564615598386e1_f64) * t25625 * t1071 * t1986 - F::cast_from(0.39512695097613069591e1_f64) * t93509 * t1097 + F::cast_from(0.39512695097613069591e1_f64) * t25695 * t3067;
    (t93490, t93514)
}
