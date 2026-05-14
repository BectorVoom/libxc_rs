//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 897/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk897<F: Float>(t9056: F, t9112: F, t9164: F, t9215: F, t106: F, t1147: F, t1182: F, t3164: F, t3171: F, t3264: F, t4410: F, t470: F, t8980: F, t8984: F, t8988: F, t8997: F, t8998: F, t9003: F) -> (F, F) {
    let t9217 = t9056 + t9112 + t9164 + t9215;
    let t9221 = 0.27818116767324025134e1 * t106 * t8980 * t470 - 0.83454350301972075402e1 * t106 * t8984 * t1182 + 0.16690870060394415081e2 * t106 * t8988 * t3171 - 0.83454350301972075402e1 * t106 * t3164 * t3264 - 0.1669087006039441508e2 * t106 * t8997 * t8998 + 0.16690870060394415081e2 * t4410 * t9003 - 0.27818116767324025134e1 * t106 * t1147 * t9217;
    (t9217, t9221)
}
