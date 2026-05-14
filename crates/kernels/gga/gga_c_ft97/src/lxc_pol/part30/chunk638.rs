//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 638/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk638<F: Float>(t10447: F, t7101: F, t1091: F, t25368: F, t2881: F, t3746: F, t6360: F, t28925: F, t296: F, t1508: F, t835: F, t6393: F, t1255: F, t6260: F, t840: F, t7131: F, t824: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29147 = t10447 * t7101;
    let t29150 = t25368 * t1091;
    let t29151 = t2881 * t29150;
    let t29154 = t6360 * t3746;
    let t29155 = t2881 * t29154;
    let t29158 = t296 * t28925;
    let t29162 = t835 * t1508 * t3746;
    let t29166 = t835 * t6393 * t1091;
    let t29170 = t840 * t1255 * t6260;
    let t29174 = t840 * t7131 * t824;
    (t29147, t29150, t29151, t29154, t29155, t29158, t29162, t29166, t29170, t29174)
}
