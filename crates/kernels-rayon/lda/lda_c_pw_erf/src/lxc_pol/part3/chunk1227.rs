//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1227/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1227(t8518: f64, t8524: f64, t8528: f64, t8545: f64, t1067: f64, t1775: f64, t1765: f64, t2737: f64, t1081: f64, t5701: f64, t1772: f64, t3007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14439 = 960.0_f64 * t8518;
    let t14440 = 192.0_f64 * t8524;
    let t14441 = 180.0_f64 * t8528;
    let t14442 = 0.0007324622014701264_f64 * t8545;
    let t14443 = t1067 * t1775;
    let t14444 = 36.0_f64 * t14443;
    let t14445 = t1765 * t2737;
    let t14446 = 0.5848223397455204_f64 * t14445;
    let t14447 = t5701 * t1081;
    let t14448 = 0.0007324622014701264_f64 * t14447;
    let t14449 = t1772 * t3007;
    (t14439, t14440, t14441, t14442, t14444, t14446, t14448, t14449)
}
