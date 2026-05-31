//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1423/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1423<F: Float>(t33: F, t3881: F, t6416: F, t1113: F, t1348: F, t20256: F, t21956: F, t2255: F, t5582: F, t21955: F, t1882: F, t1892: F, t4003: F, t5658: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t21956 * t1113 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5582 * t2255 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t21961 * t1113 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t20256);
    let t21969 = t21955 / F::cast_from(2.0_f64) + t21967 / F::cast_from(2.0_f64);
    let t21981 = t1892 * t1882;
    let t21990 = t4003 * t5658;
    (t21969, t21981, t21990)
}
