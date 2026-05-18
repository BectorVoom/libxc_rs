//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 902/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk902<F: Float>(t10151: F, t1063: F, t2343: F, t6519: F, t2268: F, t8195: F, t9189: F, t2854: F, t29975: F, t6320: F, t24139: F, t8124: F) -> (F, F, F, F) {
    let t42625 = t1063 * t2343 * t10151 * t6519;
    let t42629 = F::new(0.19918504644973304719e0) * t2268 * t9189 * t8195;
    let t42633 = F::new(0.17073003981405689759e1) * t2268 * t6320 * t2854 * t29975;
    let t42637 = F::new(0.68292015925622759036e0) * t2268 * t24139 * t8124 * t29975;
    (t42625, t42629, t42633, t42637)
}
