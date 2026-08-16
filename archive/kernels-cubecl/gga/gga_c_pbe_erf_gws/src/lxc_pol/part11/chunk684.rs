//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 684/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk684<F: Float>(t1964: F, t992: F, t2030: F, t987: F, t475: F, t2936: F, t751: F, t1: F, t1098: F, t2057: F, t2062: F, t1167: F, t804: F) -> (F, F, F, F, F, F, F) {
    let t8490 = t992 * t1964;
    let t8496 = t987 * t2030;
    let t8497 = t475 * t8496;
    let t8503 = t751 * t2936;
    let t8519 = t1098 * t2057 * t1;
    let t8520 = t8519 * t2062;
    let t8555 = t804 * t1167;
    (t8490, t8496, t8497, t8503, t8519, t8520, t8555)
}
