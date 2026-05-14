//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1017/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1017<F: Float>(t14576: F, t898: F, t338: F, t353: F, t1161: F, t3222: F, t13781: F, t3972: F, t1113: F, t9520: F, t3975: F, t9504: F, t13776: F, t13782: F, t4166: F, t9270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14577 = t898 * t14576;
    let t14579 = t338 * t353 * t14577;
    let t14582 = t1161 * param_a_c;
    let t14583 = t14582 * t3222;
    let t14584 = t13781 * t14583;
    let t14585 = t3972 * t14584;
    let t14587 = t1113 * t9520;
    let t14588 = t3975 * t14587;
    let t14589 = t3972 * t14588;
    let t14591 = t1113 * t9504;
    let t14592 = t3975 * t14591;
    let t14593 = t13776 * t14592;
    let t14595 = t1113 * t13782;
    let t14596 = t13781 * t14595;
    let t14597 = t3972 * t14596;
    let t14599 = t9270 * t4166;
    (t14577, t14579, t14582, t14584, t14585, t14588, t14589, t14592, t14593, t14596, t14597, t14599)
}
