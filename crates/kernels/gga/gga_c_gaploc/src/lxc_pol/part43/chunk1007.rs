//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1007/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1007<F: Float>(t188: F, t189: F, t193: F, t46952: F, t13830: F, t541: F, t13822: F, t1641: F, t568: F, t569: F, t574: F, t13778: F, t587: F, t589: F) -> (F, F, F, F, F) {
    let t48107 = F::new(0.35750489951850426669e0) * t188 * t189 * t46952 * t193;
    let t48109 = F::new(0.23833659967900284446e0) * t13830 * t541;
    let t48111 = F::new(0.23005755572352449806e1) * t1641 * t13822;
    let t48115 = F::new(0.23005755572352449806e1) * t574 * t568 * t569 * t46952;
    let t48121 = t587 * t589 * t13778;
    (t48107, t48109, t48111, t48115, t48121)
}
