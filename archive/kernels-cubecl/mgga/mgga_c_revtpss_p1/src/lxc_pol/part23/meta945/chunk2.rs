//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3106/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3106<F: Float>(t1161: F, t1169: F, t17089: F, t1757: F, t20521: F, t20526: F, t24331: F, t24363: F, t24366: F, t3447: F, t45080: F, t45197: F, t5120: F, t5181: F, t58317: F, t6506: F, t6535: F, t69354: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t81678: F, t81691: F, t81705: F, t81717: F, t81729: F, t81740: F, t81754: F, t81766: F) -> F {
    let t81781 = -t81128 - t81130 - t81132 - t81134 - t81136 + t81138 + F::cast_from(3.0_f64) * t5120 * t20521 + F::cast_from(0.96491876992155210402e2_f64) * t58317 * t6506 - F::cast_from(0.19298375398431042081e3_f64) * t45197 * t24331 + F::cast_from(1.0_f64) * t3447 * t24363 + F::cast_from(1.0_f64) * t1161 * (t81678 + t81691 + t81705 + t81717 + t81729 + t81740 + t81754 + t81766) * t1169 + F::cast_from(0.2069040516770936012e4_f64) * t45080 * t24366 + F::cast_from(0.17544670867903938621e1_f64) * t69354 * t1757 + F::cast_from(0.17544670867903938621e1_f64) * t20526 * t5181 + F::cast_from(0.17544670867903938621e1_f64) * t17089 * t6535;
    t81781
}
