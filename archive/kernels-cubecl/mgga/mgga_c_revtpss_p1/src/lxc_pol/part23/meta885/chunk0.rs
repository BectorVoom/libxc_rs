//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2798/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798<F: Float>(t22352: F, t2435: F, t2782: F, t4086: F, t543: F, t74965: F, t4003: F, t5744: F, t74982: F, t74700: F, t4100: F, t22394: F, t686: F, t72: F, t9680: F) -> (F, F, F, F, F) {
    let t75274 = t2435 * t22352;
    let t75298 = t2782 * t4086 * t74965 * t543;
    let t75302 = t2782 * t5744 * t74982 * t4003;
    let t75305 = t74700 * t543;
    let t75307 = t2782 * t4100 * t75305;
    let t75336 = t9680 * t22394 * t72 * t686;
    (t75274, t75298, t75302, t75307, t75336)
}
