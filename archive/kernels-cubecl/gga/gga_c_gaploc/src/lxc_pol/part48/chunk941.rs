//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 941/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk941<F: Float>(t10283: F, t2902: F, t13578: F, t16710: F, t841: F, t13483: F, t1382: F, t605: F, t23575: F, t3638: F, t13585: F, t5552: F) -> (F, F, F, F, F) {
    let t45978 = F::cast_from(2.0_f64) * t10283 * t2902;
    let t45983 = F::cast_from(24.0_f64) * t16710 * t13578 * t841;
    let t45986 = F::cast_from(2.0_f64) * t1382 * t13483 * t605;
    let t45988 = F::cast_from(2.0_f64) * t23575 * t3638;
    let t45990 = F::cast_from(2.0_f64) * t5552 * t13585;
    (t45978, t45983, t45986, t45988, t45990)
}
