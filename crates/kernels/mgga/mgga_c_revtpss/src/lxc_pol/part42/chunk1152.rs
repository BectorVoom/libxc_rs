//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1152/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1152<F: Float>(t11134: F, t11334: F, t11338: F, t11366: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15127: F, t15435: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t19103: F) -> (F,) {
    let t19125 = -0.29896666666666666667e0 * t18948 - t11334 - t11338 + 0.18257037037037037037e-1 * t19002 - 0.10954222222222222222e0 * t19004 - 0.82156666666666666667e-1 * t19007 + 0.54771111111111111111e-1 * t19009 - 0.13287407407407407408e0 * t11134 - 0.91285185185185185187e-1 * t11366 + 0.16431333333333333333e0 * t19014 - 0.27385555555555555556e-1 * t19017;
    let t19127 = -0.33218518518518518518e0 * t18906 + 0.11958666666666666667e1 * t18911 - 0.39862222222222222222e0 * t18915 + 0.3071625e0 * t18951 - 0.18257037037037037037e0 * t15123 - t15435 + 0.13287407407407407407e0 * t15127 - 0.17938e1 * t18928 + 0.11958666666666666667e1 * t18932 - 0.19931111111111111111e0 * t18939 + t19103 + 0.142419375e1 * t18980 - 0.1898925e1 * t18982 - 0.9494625e0 * t18985 - 0.76790625e-1 * t18988 + 0.3071625e0 * t18990 + 0.15358125e0 * t18993 + 0.1898925e1 * t18995 + 0.66437037037037037037e-1 * t18919 - 0.19931111111111111111e0 * t18924 + 0.99655555555555555557e-1 * t18934 + t19125;
    (t19127,)
}
