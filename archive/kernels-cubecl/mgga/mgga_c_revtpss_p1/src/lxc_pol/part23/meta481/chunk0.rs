//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1936/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1936<F: Float>(t19456: F, t996: F, t1678: F, t4746: F, t1695: F, t5015: F, t3269: F, t6343: F, t994: F) -> (F, F, F, F, F) {
    let t20188 = t996 * t19456;
    let t20191 = t4746 * t1678;
    let t20194 = t1695 * t5015;
    let t20195 = t3269 * t20194;
    let t20204 = t994 * t6343;
    (t20188, t20191, t20194, t20195, t20204)
}
