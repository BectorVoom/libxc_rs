//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 781/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk781<F: Float>(t127: F, t1664: F, t3280: F, t3282: F, t3284: F, t3288: F, t3290: F, t411: F, t5564: F, t5565: F, t5570: F, t5571: F, t5577: F, t5578: F, t5614: F, t1859: F, t443: F) -> (F, F) {
    let t5616 = t5564 + t3280 - t3282 - t3284 - t3288 - t3290 - 1.46904 * t127 * t5565 - t5570 - 29.3808 * t127 * t5571 * t1664 - t5577 + 11.75232 * t127 * t5578 * t411 + t5614;
    let t5618 = t1859 * t443;
    (t5616, t5618)
}
