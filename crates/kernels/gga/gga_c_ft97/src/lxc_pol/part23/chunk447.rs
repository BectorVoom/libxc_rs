//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 447/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk447<F: Float>(t332: F, t5478: F, t113: F, t1274: F, t992: F, t1614: F, t8: F, t40: F, t7: F) -> (F, F, F, F, F) {
    let t5479 = t5478 * t332;
    let t5480 = t5479 * t113;
    let t5483 = t1274 * t992;
    let t5567 = t8 * t1614;
    let t5585 = t40 * t7;
    (t5479, t5480, t5483, t5567, t5585)
}
