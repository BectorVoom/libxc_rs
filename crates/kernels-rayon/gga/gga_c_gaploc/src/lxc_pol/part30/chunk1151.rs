//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1151/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1151(t31126: f64, t2492: f64, t6907: f64, t9267: f64, t4811: f64, t9538: f64, t6895: f64, t4781: f64, t9274: f64, t1645: f64, t6474: f64, t1423: f64, t2326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31127 = 0.1533717038156829987e1_f64 * t31126;
    let t31129 = t9267 * t2492 * t6907;
    let t31130 = 0.72851559312449424384e1_f64 * t31129;
    let t31131 = t4811 * t9538;
    let t31132 = 0.1022478025437886658e1_f64 * t31131;
    let t31135 = 0.19171462976960374838e1_f64 * t9267 * t2492 * t6895;
    let t31144 = t4781 * t9274;
    let t31145 = 0.30674340763136599741e1_f64 * t31144;
    let t31153 = t1645 * t6474;
    let t31158 = t1423 * t2326;
    (t31127, t31130, t31132, t31135, t31145, t31153, t31158)
}
