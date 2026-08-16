//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1227/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1227<F: Float>(t8518: F, t8524: F, t8528: F, t8545: F, t1067: F, t1775: F, t1765: F, t2737: F, t1081: F, t5701: F, t1772: F, t3007: F) -> (F, F, F, F, F, F, F, F) {
    let t14439 = F::cast_from(960.0_f64) * t8518;
    let t14440 = F::cast_from(192.0_f64) * t8524;
    let t14441 = F::cast_from(180.0_f64) * t8528;
    let t14442 = F::cast_from(0.0007324622014701264_f64) * t8545;
    let t14443 = t1067 * t1775;
    let t14444 = F::cast_from(36.0_f64) * t14443;
    let t14445 = t1765 * t2737;
    let t14446 = F::cast_from(0.5848223397455204_f64) * t14445;
    let t14447 = t5701 * t1081;
    let t14448 = F::cast_from(0.0007324622014701264_f64) * t14447;
    let t14449 = t1772 * t3007;
    (t14439, t14440, t14441, t14442, t14444, t14446, t14448, t14449)
}
