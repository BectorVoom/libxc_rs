//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3121/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3121<F: Float>(t1168: F, t1187: F, t1189: F, t12429: F, t12553: F, t16997: F, t17023: F, t17026: F, t17032: F, t20606: F, t20609: F, t20668: F, t24431: F, t435: F, t5125: F, t5147: F, t58345: F, t6503: F, t6534: F, t69376: F, t69488: F, t81649: F, t81653: F, t81656: F, t81660: F, t82050: F, t82060: F, t82093: F, t82111: F) -> F {
    let t82115 = t81649 - t81653 - t81656 - t81660 + F::cast_from(0.5848223622634646207e0_f64) * t82050 * t1189 + F::cast_from(3.0_f64) * t17026 * t6503 + F::cast_from(0.30762056574649219973e4_f64) * t12553 * t6534 * t16997 * t1187 - t82060 - F::cast_from(6.0_f64) * t69488 * t5125 + F::cast_from(0.96491876992155210402e2_f64) * t69376 * t5147 + F::cast_from(18.0_f64) * t17032 * t20606 - F::cast_from(12.0_f64) * t17023 * t20609 - F::cast_from(24.0_f64) * t12429 * t24431 * t1168 + F::cast_from(0.10526802520742363173e2_f64) * t58345 * t20668 - F::cast_from(0.310907e-1_f64) * (t82093 + t82111) * t435;
    t82115
}
