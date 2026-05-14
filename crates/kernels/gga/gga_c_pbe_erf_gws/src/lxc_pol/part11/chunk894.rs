//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk894<F: Float>(t3916: F, t6677: F, t6671: F, t1114: F, t6670: F, t9847: F, t1109: F, t2105: F, t20271: F, t3765: F, t1105: F, t3880: F, t6228: F, t20490: F, t3912: F, t20281: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37750 = t3916 * t6677;
    let t37755 = t3916 * t6671;
    let t37768 = t1114 * t9847 * t6670;
    let t37800 = t2105 * t1109;
    let t37814 = t20271 * t3765;
    let t37829 = t2105 * t1105;
    let t37938 = t6228 * t3880;
    let t37965 = t3912 * t20490;
    let t37994 = t3912 * t20281;
    (t37750, t37755, t37768, t37800, t37814, t37829, t37938, t37965, t37994)
}
