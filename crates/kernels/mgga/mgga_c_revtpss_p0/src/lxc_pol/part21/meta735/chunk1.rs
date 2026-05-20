//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2585/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585<F: Float>(t10174: F, t2453: F, t9676: F, t123: F, t2434: F, t3915: F, t4131: F, t10175: F, t9686: F, t1420: F, t4075: F, t786: F) -> (F, F, F, F, F) {
    let t47520 = t2453 * t10174;
    let t47521 = t47520 * t9676;
    let t47525 = t3915 * t123 * t2434 * t4131;
    let t47527 = t10175 * t9686;
    let t47530 = t786 * t1420 * t4075;
    (t47520, t47521, t47525, t47527, t47530)
}
