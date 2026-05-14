//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1004/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1004<F: Float>(t30859: F, t675: F, t263: F, t193: F, t1425: F, t5179: F, t2354: F, t4973: F, t6003: F, t4965: F, t9744: F, t1091: F, t27991: F, t28018: F, t2: F, t21122: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30860 = t675 * t30859;
    let t30861 = t30860 * t263;
    let t30862 = t193 * t30861;
    let t30866 = t1425 * t5179;
    let t30867 = t193 * t30866;
    let t30871 = t2354 * t6003 * t4973;
    let t30875 = t9744 * t6003 * t4965;
    let t30878 = t27991 * t1091;
    let t30879 = t2354 * t30878;
    let t30883 = t2354 * t28018 * t1091;
    let t30894 = t21122 * t2;
    (t30860, t30861, t30862, t30866, t30867, t30871, t30875, t30879, t30883, t30894)
}
