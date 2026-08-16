//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2920/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920<F: Float>(t2453: F, t3908: F, t4067: F, t10115: F, t1421: F, t10168: F, t3920: F, t10174: F, t9676: F, t123: F, t2434: F, t3915: F, t4131: F) -> (F, F, F, F, F, F) {
    let t47510 = t2453 * t4067 * t3908;
    let t47512 = t10115 * t1421;
    let t47516 = t10168 * t3920;
    let t47520 = t2453 * t10174;
    let t47521 = t47520 * t9676;
    let t47525 = t3915 * t123 * t2434 * t4131;
    (t47510, t47512, t47516, t47520, t47521, t47525)
}
