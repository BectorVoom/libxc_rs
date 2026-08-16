//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 603/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk603<F: Float>(t319: F, t3746: F, t835: F, t1212: F, t824: F, t2862: F, t875: F, t840: F, t871: F, t1091: F, t882: F, t1248: F) -> (F, F, F, F, F, F, F) {
    let t4159 = t835 * t319 * t3746;
    let t4162 = t1212 * t824;
    let t4164 = t2862 * t319 * t4162;
    let t4167 = t1212 * t875;
    let t4169 = t840 * t871 * t4167;
    let t4173 = t835 * t882 * t1091;
    let t4176 = t1248 * t824;
    (t4159, t4162, t4164, t4167, t4169, t4173, t4176)
}
