//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 614/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk614<F: Float>(t172: F, t228: F, t231: F, t4995: F, t202: F, t4985: F, t237: F, t458: F, t4966: F, t4970: F, t4974: F, t236: F, t4977: F, t3724: F, t375: F, t4935: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18081 = t228 * t4995 * t172 * t231;
    let t18089 = t202 * t4985;
    let t18090 = t18089 * t237;
    let t18096 = t458 * t4966;
    let t18107 = t458 * t4970;
    let t18115 = t458 * t4974;
    let t18132 = t236 * t4977;
    let t18133 = t3724 * t18132;
    let t18145 = t89 * t375 * t4935;
    (t18081, t18089, t18090, t18096, t18107, t18115, t18132, t18133, t18145)
}
