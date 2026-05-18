//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 997/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk997<F: Float>(t447: F, t49921: F, t1445: F, t46240: F, t46244: F, t46246: F, t46250: F, t46252: F, t46257: F, t46261: F, t46264: F, t46267: F, t46271: F, t46275: F, t46283: F, t46287: F, t46289: F, t46291: F, t46294: F, t46297: F, t49874: F, t49878: F, t574: F, t597: F, t6716: F, t6717: F) -> (F, F) {
    let t50596 = t49921 * t447;
    let t50606 = F::new(0.51123901271894332901e0) * t46240 - t46244 - t46246 - t46250 - t46252 - t46257 - t46261 + F::new(0.13803453343411469884e2) * t6716 * t6717 * t50596 + t46264 - t46267 + t46271 + t46275 + t46283 - F::new(0.46011511144704899612e1) * t574 * t1445 * t49878 + F::new(0.11502877786176224903e2) * t597 * t1445 * t49874 - t46287 - t46289 - t46291 + t46294 + t46297;
    (t50596, t50606)
}
