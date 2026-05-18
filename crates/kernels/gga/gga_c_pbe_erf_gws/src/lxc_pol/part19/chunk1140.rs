//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1140/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1140<F: Float>(t14587: F, t3975: F, t3972: F, t1113: F, t9504: F, t13776: F, t13782: F, t13781: F, t1118: F, t875: F, t13796: F, t13859: F) -> (F, F, F, F, F, F, F, F) {
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
    (t14588, t14589, t14592, t14593, t14596, t14597, t14602, t14603)
}
