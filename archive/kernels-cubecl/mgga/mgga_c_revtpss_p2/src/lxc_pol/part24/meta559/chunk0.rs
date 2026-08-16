//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1678/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1678<F: Float>(t324: F, t88462: F, t88475: F, t41499: F, t41502: F, t88031: F, t11409: F, t11450: F, t11509: F, t15413: F, t1621: F, t1622: F, t1634: F, t23754: F, t23755: F, t23761: F, t2943: F, t2968: F, t3012: F, t3014: F, t41759: F, t4647: F, t6157: F, t6173: F, t6177: F, t6190: F, t6205: F, t63979: F, t78111: F, t78165: F, t88008: F, t88351: F, t88368: F, t88432: F, t88445: F, t88448: F, t88451: F) -> (F, F, F) {
    let t88477 = (t88462 + t88475) * t324;
    let t88481 = F::cast_from(0.24955700379505800916e5_f64) * t41499 * t88031 * t41502;
    let t88499 = t88368 - t88432 + F::cast_from(4.0_f64) * t4647 * t23755 + F::cast_from(0.23392894490538584828e1_f64) * t78111 * t1634 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t88351 * t3014 - F::cast_from(0.12304822629859687989e5_f64) * t41759 * t88008 * t11509 + t88445 - t88448 - t88451 - F::cast_from(0.19751673498613801407e-1_f64) * t88477 - t88481 - F::cast_from(8.0_f64) * t2943 * t1622 * t23754 - F::cast_from(0.11579025239058625248e4_f64) * t11409 * t6177 * t6173 + F::cast_from(0.12865583598954028054e3_f64) * t2968 * t78165 * t1621 + F::cast_from(0.12414243100625616072e5_f64) * t11450 * t63979 * t6157 - F::cast_from(0.14035736694323150897e2_f64) * t15413 * t23761 + F::cast_from(0.21053605041484726346e2_f64) * t3012 * t6190 * t6205;
    (t88477, t88481, t88499)
}
