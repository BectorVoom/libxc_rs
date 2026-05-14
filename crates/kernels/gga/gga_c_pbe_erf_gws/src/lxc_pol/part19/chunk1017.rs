//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1017/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1017<F: Float>(t13781: F, t14583: F, t3972: F, t1113: F, t9520: F, t3975: F, t9504: F, t13776: F, t13782: F, t1118: F, t875: F, t13796: F, t13859: F, t13972: F, t4146: F, t3166: F, t3990: F, t3991: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t14601 = t1118 * t875;
    let t14602 = t13796 * t14601;
    let t14603 = t13859 * t14602;
    let t14605 = t13972 * t4146;
    let t14608 = t3990 * t3991 * t3166;
    (t14584, t14585, t14588, t14589, t14592, t14593, t14596, t14597, t14602, t14603, t14605, t14608)
}
