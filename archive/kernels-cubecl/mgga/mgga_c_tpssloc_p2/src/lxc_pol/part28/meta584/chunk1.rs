//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1874/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1874<F: Float>(t23110: F, t23185: F, t25237: F, t23168: F, t25307: F, t10007: F, t22986: F, t4282: F, t6646: F, t25287: F, t81651: F, t13401: F, t1888: F, t22996: F) -> (F, F, F, F, F) {
    let t87601 = t23185 * t23110 * t25237;
    let t87603 = t23168 * t25307;
    let t87609 = t22986 * t6646 * t4282 * t10007;
    let t87612 = t81651 * t23110 * t25287;
    let t87615 = t1888 * t22996 * t13401;
    (t87601, t87603, t87609, t87612, t87615)
}
