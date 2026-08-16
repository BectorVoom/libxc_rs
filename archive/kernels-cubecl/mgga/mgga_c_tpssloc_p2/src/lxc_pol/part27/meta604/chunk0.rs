//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2074/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2074<F: Float>(t6787: F, t82573: F, t23384: F, t23687: F, t23658: F, t23665: F, t23494: F, t6743: F, t23547: F, t23644: F, t23647: F, t1049: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t82574 = t82573 * t6787;
    let t82576 = t23384 * t23687;
    let t82590 = t23665 * t23658;
    let t82592 = t23494 * t6743;
    let t82596 = t23547 * t6743;
    let t82605 = t23384 * t23644;
    let t82618 = t23384 * t23647;
    let t82625 = t6743 * t1049 * t883;
    (t82574, t82576, t82590, t82592, t82596, t82605, t82618, t82625)
}
