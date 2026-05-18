//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1210/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1210<F: Float>(t7284: F, t94600: F, t7243: F, t9292: F, t2022: F, t9646: F, t9648: F, t25875: F, t94394: F, t46361: F, t545: F, t1032: F, t9656: F) -> (F, F, F, F, F, F) {
    let t94602 = F::new(0.22487184191643109717e-1) * t7284 * t94600;
    let t94608 = F::new(0.17073386770573548589e-1) * t9292 * t7243;
    let t94648 = F::new(0.19637199382202157274e-3) * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94656 = t46361 * t545;
    let t94667 = t1032 * t9656;
    (t94602, t94608, t94648, t94649, t94656, t94667)
}
