//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 912/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk912<F: Float>(t2067: F, t4198: F, t30267: F, t3360: F, t7643: F, t30225: F, t438: F, t30248: F, t431: F, t30318: F, t425: F, t1195: F, t7614: F) -> (F, F, F, F, F, F, F, F) {
    let t30856 = t4198 * t2067;
    let t30861 = t3360 * t30267;
    let t30862 = t30861 * t7643;
    let t30866 = t30225 * t438;
    let t30868 = t30248 * t431;
    let t30872 = t30248 * t438;
    let t30874 = t30318 * t425;
    let t30876 = t7614 * t1195;
    (t30856, t30861, t30862, t30866, t30868, t30872, t30874, t30876)
}
