//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 673/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk673<F: Float>(t4189: F, t8186: F, t2034: F, t573: F, t2001: F, t6028: F, t7948: F, t2043: F, t570: F, t2011: F, t4261: F, t7952: F, t5752: F, t585: F, t1468: F, t2055: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8188 = 2.0 * t4189 * t8186;
    let t8189 = t2034 * t573;
    let t8191 = t6028 * t2001;
    let t8192 = t7948 * t8191;
    let t8194 = t570 * t2043;
    let t8196 = t4261 * t2011;
    let t8197 = t7952 * t8196;
    let t8199 = t5752 * t585;
    let t8201 = t1468 * t2055;
    (t8188, t8189, t8191, t8192, t8194, t8196, t8197, t8199, t8201)
}
