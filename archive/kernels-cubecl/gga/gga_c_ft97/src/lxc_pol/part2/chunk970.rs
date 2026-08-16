//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 970/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk970<F: Float>(t15010: F, t15055: F, t845: F, t91: F, t2755: F, t4226: F, t856: F, t2789: F, t4191: F, t10631: F, t1234: F, t2756: F) -> (F, F, F, F) {
    let t15056 = t15010 + t15055;
    let t15058 = t91 * t845 * t15056;
    let t15060 = t2755 * t4226;
    let t15062 = t91 * t15060 * t856;
    let t15065 = t91 * t4191 * t2789;
    let t15069 = t91 * t10631 * t1234 * t2756;
    (t15058, t15062, t15065, t15069)
}
