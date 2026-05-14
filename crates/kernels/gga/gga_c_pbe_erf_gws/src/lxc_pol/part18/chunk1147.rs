//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1147/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1147<F: Float>(t13796: F, t14423: F, t3166: F, t3989: F, t56296: F, t875: F, t14397: F, t3083: F, t1113: F, t13776: F, t3747: F, t3975: F, t810: F, t46392: F, t13781: F, t3222: F, t3886: F, t3972: F) -> (F, F, F, F, F, F) {
    let t56697 = t3989 * t13796 * t14423 * t3166;
    let t56701 = t3989 * t13796 * t56296 * t875;
    let t56703 = t3083 * t14397;
    let t56708 = t13776 * t3975 * t1113 * t3747 * t810;
    let t56717 = t13776 * t3975 * t46392;
    let t56722 = t3972 * t13781 * t3886 * param_a_c * t3222;
    (t56697, t56701, t56703, t56708, t56717, t56722)
}
