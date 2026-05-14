//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1169/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1169<F: Float>(t101761: F, t29222: F, t29231: F, t29234: F, t29237: F, t8: F, t91781: F, t91785: F, t91786: F, t93848: F, t93849: F, t93852: F, t95278: F, t95279: F, t95280: F, t95281: F, t97606: F, t97607: F, t97608: F, t99810: F, t99825: F, t99835: F) -> (F,) {
    let t101765 = -t91781 - t29231 - t91785 - t95278 - t95279 + t91786 - t95280 - t95281 - t97606 + t97607 + t8 * (t99810 + t99825 + t99835 + t101761) + t93848 - t93849 - t97608 - t29234 - t29222 - t29237 + t93852;
    (t101765,)
}
