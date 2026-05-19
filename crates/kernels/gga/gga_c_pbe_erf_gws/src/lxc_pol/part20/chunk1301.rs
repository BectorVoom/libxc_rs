//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1301/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1301<F: Float>(t1134: F, t13796: F, t13859: F, t3097: F, t1113: F, t3972: F, t3975: F, t814: F, t9847: F, t3222: F, t3721: F, t51548: F, param_a_c: F) -> (F, F, F) {
    let t56604 = t13859 * t13796 * t3097 * t1134;
    let t56613 = t3972 * t3975 * t1113 * t9847 * t814;
    let t56618 = t3972 * t51548 * t3721 * param_a_c * t3222;
    (t56604, t56613, t56618)
}
