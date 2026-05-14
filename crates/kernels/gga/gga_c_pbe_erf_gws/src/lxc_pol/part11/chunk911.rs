//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 911/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk911<F: Float>(t12575: F, t1630: F, t639: F, t12709: F, t626: F, t12701: F, t572: F, t12751: F, t5137: F, t10848: F, t2643: F, t12638: F, t12642: F, t17440: F, t1006: F, t10485: F) -> (F, F, F, F, F, F, F, F) {
    let t41447 = t639 * t1630 * t12575;
    let t41459 = t12709 * t626;
    let t41514 = t12701 * t572;
    let t41524 = t639 * t5137 * t12751;
    let t41562 = t10848 * t2643;
    let t41570 = t639 * t1630 * t12638;
    let t41573 = t639 * t17440 * t12642;
    let t41595 = t1006 * t10485;
    (t41447, t41459, t41514, t41524, t41562, t41570, t41573, t41595)
}
