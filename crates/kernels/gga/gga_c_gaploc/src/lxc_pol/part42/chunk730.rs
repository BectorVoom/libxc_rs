//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 730/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk730<F: Float>(t121: F, t5745: F, t2084: F, t321: F, t2088: F, t324: F, t304: F, t330: F, t5557: F, t123: F, t160: F, t4348: F, t498: F) -> (F, F, F, F, F, F, F) {
    let t16534 = t121 * t5745;
    let t16687 = t2084 * t321;
    let t16692 = F::new(1.0) / t2088 / t324;
    let t16710 = t304 / t5557 / t330;
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t17288 = t498 * t4348;
    (t16534, t16687, t16692, t16710, t16879, t16880, t17288)
}
