//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1270/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1270<F: Float>(t3143: F, t7135: F, t25625: F, t7166: F, t11213: F, t1976: F, t11711: F, t25517: F, t11865: F, t25516: F, t11874: F, t27492: F) -> (F, F, F, F, F, F) {
    let t93516 = t3143 * t7135;
    let t93521 = t25625 * t7166;
    let t93528 = t11213 * t1976;
    let t93541 = t25517 * t11711;
    let t93543 = t11865 * t25516;
    let t93548 = t11874 * t27492;
    (t93516, t93521, t93528, t93541, t93543, t93548)
}
