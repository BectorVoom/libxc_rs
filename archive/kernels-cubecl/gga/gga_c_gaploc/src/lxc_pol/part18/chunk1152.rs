//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1152/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1152<F: Float>(t31126: F, t2492: F, t6907: F, t9267: F, t4811: F, t9538: F, t6895: F, t4781: F, t9274: F, t1645: F, t6474: F, t1423: F, t2326: F) -> (F, F, F, F, F, F, F) {
    let t31127 = F::cast_from(0.1533717038156829987e1_f64) * t31126;
    let t31129 = t9267 * t2492 * t6907;
    let t31130 = F::cast_from(0.72851559312449424384e1_f64) * t31129;
    let t31131 = t4811 * t9538;
    let t31132 = F::cast_from(0.1022478025437886658e1_f64) * t31131;
    let t31135 = F::cast_from(0.19171462976960374838e1_f64) * t9267 * t2492 * t6895;
    let t31144 = t4781 * t9274;
    let t31145 = F::cast_from(0.30674340763136599741e1_f64) * t31144;
    let t31153 = t1645 * t6474;
    let t31158 = t1423 * t2326;
    (t31127, t31130, t31132, t31135, t31145, t31153, t31158)
}
