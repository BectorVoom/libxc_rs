//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1142/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142<F: Float>(t532: F, t8107: F, t1450: F, t2107: F, t5542: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t4248: F, t508: F, t569: F, t651: F, t7359: F, t7732: F, t7898: F, t7969: F, t7978: F, t7984: F, t7988: F, t8065: F, t8075: F, t8079: F) -> (F, F, F, F) {
    let t8108 = t532 * t8107;
    let t8109 = t8108 * t1450;
    let t8111 = t2107 * t5542;
    let t8113 = -t118 * t8065 - t1502 * t2089 - F::cast_from(2.0_f64) * t1519 * t7359 - t1843 * t2052 + t1911 * t2093 + F::cast_from(3.0_f64) * t2014 * t8079 + t2014 * t8109 - t2014 * t8111 - F::cast_from(2.0_f64) * t2056 * t4248 - F::cast_from(2.0_f64) * t2056 * t7732 + t2108 * t7898 - t508 * t7969 + t569 * t8075 - F::cast_from(2.0_f64) * t651 * t7978 - F::cast_from(2.0_f64) * t651 * t7984 - F::cast_from(2.0_f64) * t651 * t7988;
    (t8108, t8109, t8111, t8113)
}
