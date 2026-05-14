//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 710/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk710<F: Float>(t2558: F, t33232: F, t9647: F, t13176: F, t731: F, t13225: F, t2549: F, t2562: F, t32179: F, t883: F, t943: F, t33289: F, t7810: F, t9889: F, t13055: F, t28073: F) -> (F, F, F, F, F, F) {
    let t43224 = t9647 * t33232 * t2558;
    let t43290 = t731 * t13176;
    let t43326 = t2549 * t13225;
    let t43330 = t943 * t2562 * t883 * t32179;
    let t43363 = t7810 * t33289 * t9889;
    let t43370 = t28073 * t13055;
    (t43224, t43290, t43326, t43330, t43363, t43370)
}
