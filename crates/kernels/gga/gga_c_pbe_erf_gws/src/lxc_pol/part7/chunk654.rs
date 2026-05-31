//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 654/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk654<F: Float>(t1806: F, t579: F, t1730: F, t1798: F, t1734: F, t582: F, t616: F, t596: F, t188: F, t1804: F, t610: F, t186: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5168 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t579 * t1806;
    let t5169 = t1730 * t1798;
    let t5170 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5169;
    let t5171 = t582 * t1734;
    let t5172 = t616 * t5171;
    let t5173 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5172;
    let t5174 = t596 * t596;
    let t5175 = F::cast_from(1.0_f64) / t5174;
    let t5176 = t188 * t5175;
    let t5177 = t1804 * t610;
    let t5178 = t5176 * t5177;
    let t5179 = t186 * t5178;
    (t5168, t5170, t5171, t5173, t5174, t5175, t5177, t5178, t5179)
}
