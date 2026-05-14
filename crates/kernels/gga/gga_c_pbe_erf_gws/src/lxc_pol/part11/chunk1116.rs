//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1116<F: Float>(t1105: F, t1109: F, t1115: F, t12111: F, t13142: F, t13174: F, t13182: F, t13217: F, t2376: F, t2401: F, t2408: F, t2409: F, t2501: F, t3047: F, t3052: F, t3055: F, t3207: F, t335: F, t338: F, t353: F, t3703: F, t3717: F, t3722: F, t3733: F, t376: F, t3772: F, t3886: F, t3896: F, t3907: F, t3921: F, t43451: F, t4386: F, t46656: F, t46667: F, t49955: F, t829: F, t830: F, t8589: F, t8629: F, t9815: F) -> (F,) {
    let t50440 = t2408 * t2409 * t2376 * t3717 * t3886 / 8.0 - 3.0 / 4.0 * t3207 * t2409 * t8589 * t13182 - 3.0 / 8.0 * t3207 * t2409 * t2376 * t3703 * t3886 + t8629 * t4386 * t353 * t3896 * t1109 / 8.0 + t8629 * t4386 * t353 * t43451 * t1105 / 12.0 + t335 * t338 * t3907 * t3722 / 8.0 + 3.0 / 16.0 * t2401 * t338 * t353 * t376 * t49955 - t1115 * t46656 / 4.0 - t3055 * t829 * t830 * t2501 * t3772 / 24.0 - t9815 * t13217 / 32.0 + t3921 * t12111 / 8.0 - t46667 * t3733 / 32.0 - t13142 * t3052 / 12.0 - t13174 * t3047 / 24.0;
    (t50440,)
}
