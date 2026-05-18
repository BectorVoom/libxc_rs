//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 467/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk467<F: Float>(t2172: F, t858: F, t862: F, t361: F, t861: F) -> (F, F, F, F) {
    let t2246 = F::new(0.22831111111111111111e-1) * t2172;
    let t2252 = t858 * t862;
    let t2255 = t861 * t361;
    let t2256 = F::new(1.0) / t2255;
    (t2246, t2252, t2255, t2256)
}
