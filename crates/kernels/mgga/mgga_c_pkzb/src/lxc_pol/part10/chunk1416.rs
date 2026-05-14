//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1416/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1416<F: Float>(t10365: F, t2464: F, t3282: F, t955: F, t1306: F, t27850: F, t27905: F, t27908: F, t27960: F, t27963: F, t27965: F, t27967: F, t27971: F, t27974: F, t27978: F, t27980: F, t27982: F, t8572: F) -> (F,) {
    let t28595 = t10365 * t2464;
    let t28599 = t955 * t3282;
    let t28603 = -2.0 * t1306 * t28595 * t955 + 8.0 * t1306 * t28599 * t8572 + t27850 + t27905 + t27908 - t27960 - t27963 - t27965 - t27967 + t27971 - t27974 + t27978 - t27980 - t27982;
    (t28603,)
}
