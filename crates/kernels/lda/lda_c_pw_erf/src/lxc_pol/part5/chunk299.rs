//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 299/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk299<F: Float>(t1012: F, t1030: F, t1001: F, t1005: F, t1011: F, t1013: F, t1023: F, t1028: F, t174: F, t365: F, t372: F, t380: F, t387: F, t4: F, t474: F, t71: F, t84: F, t910: F, t916: F, t938: F, t966: F, t972: F, t974: F, t984: F, t989: F, t992: F, t997: F) -> (F, F) {
    let t1031 = t1012 * t1030;
    let t1034 = -F::cast_from(0.0007098192444444445_f64) * t4 * t474 * t71 - F::cast_from(0.03424666666666667_f64) * t174 * t966 * t372 - F::new(2.0) * t972 * t974 + F::new(1.0) * t365 * t984 + F::cast_from(32.1646831778707_f64) * t989 * t992 + t997 + t1001 + t916 - t938 - t910 - F::cast_from(0.0002441540671567088_f64) * t4 * t474 * t84 - F::cast_from(0.010843580882781523_f64) * t174 * t1005 * t387 - F::cast_from(1.169644679491041_f64) * t1011 * t1013 + F::cast_from(0.5848223397455204_f64) * t380 * t1023 + F::cast_from(17.315755899375862_f64) * t1028 * t1031;
    (t1031, t1034)
}
