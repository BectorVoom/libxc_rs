//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1083/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1083<F: Float>(t105: F, t169: F, t172: F, t452: F, t46952: F, t42756: F, t42759: F, t42763: F, t42767: F, t42771: F, t42772: F, t42773: F, t42774: F, t42778: F, t42782: F) -> F {
    let t46991 = F::cast_from(0.28455006635676149599e-1_f64) * t105 * t452 * t46952 * t169 * t172;
    let t46996 = -t42756 + t46991 + F::cast_from(0.28455006635676149599e-1_f64) * t42759 + t42763 + t42767 - t42771 - t42772 + t42773 - F::cast_from(0.15808337019820083111e-2_f64) * t42774 - F::cast_from(0.19918504644973304719e0_f64) * t42778 + F::cast_from(0.34146007962811379518e0_f64) * t42782;
    t46996
}
