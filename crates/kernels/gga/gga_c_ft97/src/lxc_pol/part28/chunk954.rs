//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 954/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk954<F: Float>(t1557: F, t7211: F, t174: F, t2248: F, t7238: F, t7244: F, t32107: F, t376: F, t5665: F, t1317: F, t32099: F, t32102: F) -> (F, F, F, F, F, F, F) {
    let t137082 = t7211 * t1557;
    let t137087 = t2248 * t174;
    let t137089 = t7238 * t137087 * t7244;
    let t137090 = F::new(10.0) / F::new(27.0) * t137089;
    let t137102 = t5665 * t376 * t32107;
    let t137105 = t1317 * t376 * t32099;
    let t137108 = t1317 * t376 * t32102;
    (t137082, t137087, t137089, t137090, t137102, t137105, t137108)
}
