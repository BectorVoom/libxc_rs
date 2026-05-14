//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 287/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk287<F: Float>(t1137: F, t1169: F, t1173: F, t1175: F, t247: F, t263: F, t792: F, t992: F, t666: F, t89: F, t1095: F, t801: F, t278: F, t274: F, t807: F, t291: F) -> (F, F, F, F, F, F, F) {
    let t1178 = -t1137 * t263 - t1173 * t247 - 2.0 * t1169 + 2.0 * t1175;
    let t1186 = t792 * t992;
    let t1188 = t89 * t666 * t1186;
    let t1190 = t801 * t1095;
    let t1193 = t1095 * t278;
    let t1196 = 0.23410285231011484e0 * t1190 * t274 - 0.532971647967385935e-1 * t807 * t1193;
    let t1197 = t291 * t1196;
    (t1178, t1186, t1188, t1190, t1193, t1196, t1197)
}
