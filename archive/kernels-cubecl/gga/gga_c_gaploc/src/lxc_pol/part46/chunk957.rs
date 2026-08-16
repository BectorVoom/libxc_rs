//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 957/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk957<F: Float>(t42944: F, t5241: F, t5640: F, t590: F, t33289: F, t7810: F, t9889: F, t2028: F, t3038: F, t787: F, t9636: F, t13055: F, t28073: F) -> (F, F, F, F) {
    let t43361 = F::cast_from(0.13803453343411469884e2_f64) * t5640 * t5241 * t42944 * t590;
    let t43363 = t7810 * t33289 * t9889;
    let t43364 = F::cast_from(0.19171462976960374838e1_f64) * t43363;
    let t43368 = F::cast_from(0.39722766613167140743e-1_f64) * t787 * t9636 * t3038 * t2028;
    let t43370 = t28073 * t13055;
    (t43361, t43364, t43368, t43370)
}
