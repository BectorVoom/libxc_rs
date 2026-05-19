//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1134/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1134<F: Float>(t14565: F, t14567: F, t1135: F, t3065: F, t2134: F, t1161: F, t3222: F, t13781: F, t3972: F, t1113: F, t9520: F, t3975: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t14568 = t14565 * t14567;
    let t14570 = t3065 * t1135;
    let t14571 = t2134 * t14570;
    let t14582 = t1161 * param_a_c;
    let t14583 = t14582 * t3222;
    let t14584 = t13781 * t14583;
    let t14585 = t3972 * t14584;
    let t14587 = t1113 * t9520;
    let t14588 = t3975 * t14587;
    (t14568, t14570, t14571, t14582, t14584, t14585, t14588)
}
