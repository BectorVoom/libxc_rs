//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1631/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1631<F: Float>(t3584: F, t3601: F, t12657: F, t1284: F, t3624: F, t12875: F, t12916: F, t5331: F, t12871: F, t5340: F, t3568: F, t1222: F, t12282: F, t17471: F) -> (F, F, F, F, F, F) {
    let t44759 = t3584 * t3601;
    let t44769 = t12657 * t1284 * t3624;
    let t44773 = t5331 * t12916 * t12875;
    let t44776 = t5340 * t12916 * t12871;
    let t44778 = t3584 * t3568;
    let t44786 = t1222 * t17471 * t12282;
    (t44759, t44769, t44773, t44776, t44778, t44786)
}
