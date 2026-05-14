//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 780/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk780<F: Float>(t46094: F, t6710: F, t6711: F, t37679: F, t6590: F, t10370: F, t10615: F, t1457: F, t44294: F, t447: F, t204: F, t2476: F, t10241: F, t2754: F) -> (F, F, F, F, F, F) {
    let t46097 = 0.43710935587469654631e2 * t6710 * t6711 * t46094;
    let t46098 = t37679 * t6590;
    let t46102 = 0.50050685932590597338e1 * t10615 * t1457 * t10370;
    let t46103 = t44294 * t447;
    let t46106 = 0.46011511144704899612e1 * t2476 * t204 * t46103;
    let t46115 = t10241 * t2754;
    (t46097, t46098, t46102, t46103, t46106, t46115)
}
