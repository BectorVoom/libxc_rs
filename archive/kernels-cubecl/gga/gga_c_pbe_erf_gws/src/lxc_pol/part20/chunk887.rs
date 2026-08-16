//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 887/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk887<F: Float>(t2358: F, t3916: F, t3703: F, t831: F, t6148: F, t830: F, t1109: F, t2395: F, t829: F, t3028: F, t1145: F, t858: F) -> (F, F, F, F, F, F) {
    let t9815 = t3916 * t2358;
    let t9818 = t831 * t3703;
    let t9820 = t6148 * t830 * t9818;
    let t9827 = t829 * t830 * t2395 * t1109;
    let t9832 = t829 * t830 * t831 * t3028;
    let t9837 = t858 * t1145;
    (t9815, t9818, t9820, t9827, t9832, t9837)
}
