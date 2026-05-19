//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1155/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1155<F: Float>(t42999: F, t43003: F, t43006: F, t43010: F, t43014: F, t43017: F, t43019: F, t43023: F, t43028: F, t43032: F, t43035: F, t13945: F, t681: F) -> (F, F) {
    let t47625 = F::cast_from(0.20508069947045931423e-1_f64) * t42999 + F::cast_from(0.15381052460284448567e-1_f64) * t43003 + t43006 - F::cast_from(0.17090058289204942852e-2_f64) * t43010 - t43014 - t43017 + t43019 - t43023 + t43028 + t43032 - F::cast_from(0.85450291446024714263e-3_f64) * t43035;
    let t47629 = F::cast_from(0.76905262301422242837e-2_f64) * t681 * t13945;
    (t47625, t47629)
}
