//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1135/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1135<F: Float>(t13879: F, t2009: F, t773: F, t38950: F, t955: F, t43904: F, t43908: F, t43909: F, t43910: F, t43911: F, t43913: F, t43915: F, t43918: F, t43919: F, t43922: F) -> F {
    let t47430 = F::cast_from(0.35750489951850426669e0_f64) * t773 * t13879 * t2009;
    let t47432 = t955 * t38950;
    let t47436 = -t47430 - F::cast_from(0.25561950635947166451e0_f64) * t43904 + t43908 + F::cast_from(0.23833659967900284446e0_f64) * t47432 - t43909 + t43910 + t43911 - t43913 + t43915 + t43918 - F::cast_from(0.19171462976960374838e0_f64) * t43919 - F::cast_from(0.19171462976960374838e0_f64) * t43922;
    t47436
}
