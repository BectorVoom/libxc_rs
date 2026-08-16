//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 815/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk815<F: Float>(t1832: F, t411: F, t1568: F, t756: F, t1697: F, t9: F, t1664: F, t1870: F, t1871: F, t3349: F, t3351: F, t5505: F, t5517: F, t5520: F, t5560: F, t5561: F, t5562: F, t5563: F, t5641: F) -> (F, F, F, F, F) {
    let t5643 = t1832 * t411;
    let t5647 = t756 * t1568;
    let t5651 = t9 * t1697;
    let t5652 = t756 * t1664;
    let t5658 = -t5505 + t5517 + t5520 - F::cast_from(3.44851_f64) * t5641 + F::cast_from(10.34553_f64) * t1870 * t1871 * t5643 + F::cast_from(5.172765_f64) * t1870 * t1871 * t5647 - F::cast_from(20.69106_f64) * t1870 * t5651 * t5652 + t5560 + t5561 - t5562 - t5563 - F::cast_from(1.532671111111111_f64) * t3349 + F::cast_from(0.5747516666666667_f64) * t3351;
    (t5643, t5647, t5651, t5652, t5658)
}
