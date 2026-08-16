//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1051/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1051<F: Float>(t35328: F, t6109: F, t681: F, t150042: F, t41825: F, t446: F, t1131: F, t33452: F, t1434: F, t193: F, t2506: F, t35354: F) -> (F, F, F, F, F) {
    let t151017 = t6109 * t681 * t35328;
    let t151020 = t446 * t41825 * t150042;
    let t151022 = t33452 * t1131;
    let t151025 = t1434 * t193 * t2506 * t151022;
    let t151027 = t1434 * t681 * t35354;
    (t151017, t151020, t151022, t151025, t151027)
}
