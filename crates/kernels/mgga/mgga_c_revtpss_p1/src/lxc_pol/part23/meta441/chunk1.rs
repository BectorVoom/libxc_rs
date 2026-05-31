//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1858/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1858<F: Float>(t11134: F, t11334: F, t11338: F, t11366: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15127: F, t15435: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t19103: F) -> F {
    let t19125 = -F::cast_from(0.29896666666666666667e0_f64) * t18948 - t11334 - t11338 + F::cast_from(0.18257037037037037037e-1_f64) * t19002 - F::cast_from(0.10954222222222222222e0_f64) * t19004 - F::cast_from(0.82156666666666666667e-1_f64) * t19007 + F::cast_from(0.54771111111111111111e-1_f64) * t19009 - F::cast_from(0.13287407407407407408e0_f64) * t11134 - F::cast_from(0.91285185185185185187e-1_f64) * t11366 + F::cast_from(0.16431333333333333333e0_f64) * t19014 - F::cast_from(0.27385555555555555556e-1_f64) * t19017;
    let t19127 = -F::cast_from(0.33218518518518518518e0_f64) * t18906 + F::cast_from(0.11958666666666666667e1_f64) * t18911 - F::cast_from(0.39862222222222222222e0_f64) * t18915 + F::cast_from(0.3071625e0_f64) * t18951 - F::cast_from(0.18257037037037037037e0_f64) * t15123 - t15435 + F::cast_from(0.13287407407407407407e0_f64) * t15127 - F::cast_from(0.17938e1_f64) * t18928 + F::cast_from(0.11958666666666666667e1_f64) * t18932 - F::cast_from(0.19931111111111111111e0_f64) * t18939 + t19103 + F::cast_from(0.142419375e1_f64) * t18980 - F::cast_from(0.1898925e1_f64) * t18982 - F::cast_from(0.9494625e0_f64) * t18985 - F::cast_from(0.76790625e-1_f64) * t18988 + F::cast_from(0.3071625e0_f64) * t18990 + F::cast_from(0.15358125e0_f64) * t18993 + F::cast_from(0.1898925e1_f64) * t18995 + F::cast_from(0.66437037037037037037e-1_f64) * t18919 - F::cast_from(0.19931111111111111111e0_f64) * t18924 + F::cast_from(0.99655555555555555557e-1_f64) * t18934 + t19125;
    t19127
}
