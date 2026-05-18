//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 610/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk610<F: Float>(t1240: F, t870: F, t875: F, t296: F, t1248: F, t2749: F, t824: F, t992: F, t2875: F, t2874: F, t2882: F, t2881: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4246 = t1240 * t870;
    let t4247 = t4246 * t875;
    let t4248 = t296 * t4247;
    let t4251 = t2749 * t1248;
    let t4252 = t296 * t4251;
    let t4255 = t992 * t824;
    let t4256 = t2875 * t4255;
    let t4257 = t2874 * t4256;
    let t4260 = t992 * t875;
    let t4261 = t2882 * t4260;
    let t4262 = t2881 * t4261;
    (t4246, t4247, t4248, t4251, t4252, t4256, t4257, t4261, t4262)
}
