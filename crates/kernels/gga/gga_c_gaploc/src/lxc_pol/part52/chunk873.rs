//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 873/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk873<F: Float>(t2894: F, t42381: F, t46703: F, t46704: F, t46705: F, t46708: F, t46709: F, t46715: F, t46717: F, t46724: F, t46730: F, t46732: F, t46735: F, t46740: F, t46742: F, t46754: F, t46758: F, t46760: F, t46765: F, t47838: F, t48178: F) -> (F,) {
    let t50763 = t46703 + t46704 + t46705 + 0.23833659967900284447e0 * t47838 * t2894 - 0.76685851907841499354e0 * t48178 - t46708 - 0.10427226235956374445e0 * t46709 + t42381 - t46715 + t46717 - t46724 - t46730 + t46732 + t46735 + t46740 + t46742 - t46754 - t46758 - t46760 + t46765;
    (t50763,)
}
