//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1281/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1281<F: Float>(t119778: F, t23667: F, t5899: F, t1017: F, t2185: F, t23657: F, t3526: F, t5900: F, t1039: F, t3408: F, t30105: F, t358: F, t1969: F, t363: F, t446: F, t18: F, t27034: F, t3281: F) -> (F, F, F, F, F) {
    let t119906 = t5899 * t23667 * t119778;
    let t119913 = t23657 * t2185 * t5900 * t3526 * t1017;
    let t119917 = t23657 * t2185 * t5900 * t1039 * t3408;
    let t119919 = t30105 * t358;
    let t119922 = t446 * t1969 * t119919 * t363;
    let t119926 = t3281 * t1969 * t27034 * t18;
    (t119906, t119913, t119917, t119922, t119926)
}
