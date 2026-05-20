//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1336/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1336<F: Float>(t12349: F, t12352: F, t16708: F, t16893: F, t16929: F, t16931: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F, t12261: F, t12297: F, t16706: F, t16869: F, t16873: F, t16876: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t20425: F) -> F {
    let t20445 = -F::new(0.76790625e-1) * t20366 + F::new(0.3071625e0) * t20368 + F::new(0.15358125e0) * t20371 - t16893 - t12349 - t12352 + F::new(0.3071625e0) * t20373 - t16929 + F::cast_from(0.13287407407407407407e0_f64) * t16708 + F::cast_from(0.36514074074074074073e-1_f64) * t16931 + F::cast_from(0.36514074074074074075e-1_f64) * t20378;
    let t20447 = F::cast_from(0.91285185185185185187e-1_f64) * t12261 - t16869 - t16873 - F::cast_from(0.27385555555555555556e-1_f64) * t20268 + F::cast_from(0.26574814814814814815e0_f64) * t16706 + F::cast_from(0.18257037037037037037e0_f64) * t16876 + F::cast_from(0.82156666666666666667e-1_f64) * t20274 + F::cast_from(0.18257037037037037037e-1_f64) * t20276 - F::cast_from(0.10954222222222222222e0_f64) * t20278 - F::cast_from(0.54771111111111111111e-1_f64) * t20280 + t20425 + F::new(0.1898925e1) * t20338 + F::cast_from(0.16431333333333333333e0_f64) * t20341 - F::cast_from(0.54771111111111111112e-1_f64) * t20344 - F::cast_from(0.16431333333333333333e0_f64) * t20347 + F::cast_from(0.32862666666666666666e0_f64) * t20350 + F::cast_from(0.49293999999999999999e0_f64) * t20353 + F::cast_from(0.13287407407407407408e0_f64) * t12297 + F::cast_from(0.142419375e1_f64) * t20357 - F::new(0.1898925e1) * t20359 - F::new(0.9494625e0) * t20362 + t20445;
    t20447
}
