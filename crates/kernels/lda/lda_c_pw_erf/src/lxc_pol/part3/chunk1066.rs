//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1066/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1066<F: Float>(t8518: F, t8524: F, t8528: F, t8545: F, t1067: F, t1775: F, t1765: F, t2737: F, t1081: F, t5701: F, t1772: F, t3007: F, t8527: F, t8533: F, t8536: F, t8539: F, t8542: F, t8716: F, t8733: F, t8737: F, t8740: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14439 = 960.0 * t8518;
    let t14440 = 192.0 * t8524;
    let t14441 = 180.0 * t8528;
    let t14442 = 0.0007324622014701264 * t8545;
    let t14443 = t1067 * t1775;
    let t14444 = 36.0 * t14443;
    let t14445 = t1765 * t2737;
    let t14446 = 0.5848223397455204 * t14445;
    let t14447 = t5701 * t1081;
    let t14448 = 0.0007324622014701264 * t14447;
    let t14449 = t1772 * t3007;
    let t14450 = 0.0005696928233656539 * t14449;
    let t14451 = -t14439 + t14440 + t8527 + t14441 + t8533 - t8536 + t8539 - t8542 + t14442 + t14444 - t14446 + t14448 - t14450 + t8733 - t8716 - t8737 + t8740;
    (t14439, t14440, t14441, t14442, t14444, t14446, t14448, t14450, t14451)
}
