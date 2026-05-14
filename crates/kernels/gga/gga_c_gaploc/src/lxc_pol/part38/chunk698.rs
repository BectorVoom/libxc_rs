//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 698/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk698<F: Float>(t10205: F, t871: F, t2748: F, t3113: F, t12964: F, t2487: F, t6985: F, t10615: F, t1423: F, t3129: F, t40377: F, t2890: F, t9267: F, t9278: F, t20671: F, t31047: F, t34814: F) -> (F, F, F, F, F, F, F) {
    let t42114 = t10205 * t871;
    let t42115 = t2748 * t3113;
    let t42146 = t2487 * t6985 * t12964;
    let t42156 = t10615 * t1423 * t3129;
    let t42170 = 0.19171462976960374838e0 * t40377;
    let t42183 = t9267 * t2890 * t9278;
    let t42187 = t31047 * t20671 * t34814;
    (t42114, t42115, t42146, t42156, t42170, t42183, t42187)
}
