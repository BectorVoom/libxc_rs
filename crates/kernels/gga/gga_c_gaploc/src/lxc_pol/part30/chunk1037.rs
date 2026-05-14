//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1037/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1037<F: Float>(t20369: F, t31119: F, t31120: F, t883: F, t6907: F, t888: F, t9263: F, t2492: F, t9267: F, t4811: F, t9538: F, t6895: F, t4781: F, t9274: F, t1645: F, t6474: F) -> (F, F, F, F, F, F, F) {
    let t31124 = 0.46011511144704899612e1 * t31119 * t31120 * t883 * t20369;
    let t31126 = t9263 * t888 * t6907;
    let t31127 = 0.1533717038156829987e1 * t31126;
    let t31129 = t9267 * t2492 * t6907;
    let t31130 = 0.72851559312449424384e1 * t31129;
    let t31131 = t4811 * t9538;
    let t31132 = 0.1022478025437886658e1 * t31131;
    let t31135 = 0.19171462976960374838e1 * t9267 * t2492 * t6895;
    let t31144 = t4781 * t9274;
    let t31145 = 0.30674340763136599741e1 * t31144;
    let t31153 = t1645 * t6474;
    (t31124, t31127, t31130, t31132, t31135, t31145, t31153)
}
