//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 794/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk794<F: Float>(t24663: F, t24729: F, t24788: F, t24848: F, t24650: F, t258: F, t1403: F, t1427: F, t1454: F, t2331: F, t24253: F, t24257: F, t24398: F, t24403: F, t24405: F, t24408: F, t24410: F, t24413: F, t24416: F, t24419: F, t24421: F, t24425: F, t24430: F, t24565: F, t247: F, t5996: F, t6011: F, t6064: F, t6068: F) -> (F, F, F) {
    let t24850 = t24663 + t24729 + t24788 + t24848;
    let t24852 = t24650 * t258;
    let t24856 = t5996 * t6068 / 3.0 - t24253 / 9.0 + t24257 * t1427 / 6.0 + t1403 * t24398 / 6.0 + t5996 * t6064 / 3.0 - 4.0 * t24403 - 2.0 * t24405 - 12.0 * t24408 - 2.0 * t24410 + 4.0 * t24413 + 4.0 * t24416 + 8.0 * t24419 + 8.0 * t24421 - 2.0 / 3.0 * t1403 * t24425 - t2331 * t1454 - 4.0 * t24430 - 2.0 * t24565 - t247 * t24850 + 2.0 * t24852 - 2.0 / 3.0 * t5996 * t6011;
    (t24850, t24852, t24856)
}
