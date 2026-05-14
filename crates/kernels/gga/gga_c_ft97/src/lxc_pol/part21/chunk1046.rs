//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1046/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1046<F: Float>(t108: F, t7800: F, t358: F, t984: F, t22914: F, t25606: F, t25612: F, t25863: F, t23089: F, t6414: F, t5617: F, t25545: F, t5495: F, t1284: F, t1900: F, t7149: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100050 = t108 * t7800;
    let t100055 = t984 * t358;
    let t100065 = 2.0 / 27.0 * t22914 * t25606;
    let t100067 = 2.0 / 27.0 * t22914 * t25612;
    let t100079 = t22914 * t25863 / 27.0;
    let t100085 = t6414 * t23089 / 9.0;
    let t100089 = t5617 * t984;
    let t100099 = t5495 * t25545 / 9.0;
    let t100127 = t1284 * t7149 * t1900;
    (t100050, t100055, t100065, t100067, t100079, t100085, t100089, t100099, t100127)
}
