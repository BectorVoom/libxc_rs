//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1156/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1156<F: Float>(t3013: F, t8572: F, t2985: F, t3017: F, t3021: F, t2988: F, t8680: F, t1032: F, t8685: F, t8689: F, t2991: F, t3016: F, t375: F, t26153: F, t8688: F, t522: F, t8621: F) -> (F, F, F, F, F, F) {
    let t26237 = 6.0 * t8572 * t3013;
    let t26238 = t2985 * t3017;
    let t26240 = 0.96490945932906628932e2 * t26238 * t3021;
    let t26242 = 4.0 * t2988 * t8680;
    let t26243 = t1032 * t8685;
    let t26245 = 0.20690005882282467367e4 * t26243 * t8689;
    let t26248 = t375 / t3016 / t2991;
    let t26251 = 0.620700176468474021e4 * t26248 * t26153 * t8688;
    let t26252 = t522 * t8621;
    (t26237, t26240, t26242, t26245, t26251, t26252)
}
