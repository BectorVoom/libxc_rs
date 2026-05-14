//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 223/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk223<F: Float>(t14: F, t852: F, t1: F, t269: F, t546: F, t106: F, t257: F, t748: F, t10: F, t103: F, t164: F, t266: F, t303: F, t304: F, t758: F, t849: F) -> (F, F, F, F, F, F) {
    let t853 = t852 * t14;
    let t854 = t269 * t1;
    let t855 = t854 * t546;
    let t858 = t106 * t257;
    let t859 = t858 * t748;
    let t868 = 0.58998125e-2 * t849 * t304 - 0.11799625e-1 * t853 * t855 - 0.58998125e-2 * t303 * t859 - 0.14341111111111111111e-1 * t103 * t10 * t266 - 0.21511666666666666667e-1 * t103 * t164 * t758;
    (t853, t854, t855, t858, t859, t868)
}
