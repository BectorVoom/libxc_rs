//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1313/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1313<F: Float>(t19456: F, t996: F, t1678: F, t4746: F, t1695: F, t5015: F, t3269: F, t6343: F, t994: F, t19462: F, t378: F, t4772: F) -> (F, F, F, F, F, F) {
    let t20188 = t996 * t19456;
    let t20191 = t4746 * t1678;
    let t20194 = t1695 * t5015;
    let t20195 = t3269 * t20194;
    let t20204 = t994 * t6343;
    let t20211 = t19462 * t378;
    let t20214 = t4772 * t1695;
    (t20188, t20191, t20195, t20204, t20211, t20214)
}
