//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1490/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1490<F: Float>(t22245: F, t808: F, t9736: F, t22236: F, t6884: F, t9741: F, t14104: F, t47856: F, t2439: F, t3895: F, t6896: F, t136: F, t2457: F, t47480: F, t6895: F) -> (F, F, F, F, F, F) {
    let t74711 = t9736 * t808 * t22245;
    let t74714 = t9736 * t808 * t22236;
    let t74717 = t9741 * t6884;
    let t74733 = t47856 * t14104;
    let t74757 = t2439 * t3895 * t6896;
    let t74770 = t47480 * t6895 * t136 * t2457;
    (t74711, t74714, t74717, t74733, t74757, t74770)
}
