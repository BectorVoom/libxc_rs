//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1981/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1981<F: Float>(t98235: F, t98238: F, t98243: F, t94485: F, t94498: F, t94501: F, t94503: F, t94505: F, t94509: F, t94511: F, t96326: F, t98245: F, t98253: F) -> F {
    let t102534 = F::cast_from(0.22866142996303859718e-3_f64) * t98235;
    let t102535 = F::cast_from(0.57165357490759649296e-4_f64) * t98238;
    let t102537 = F::cast_from(0.2032800112371413129e-3_f64) * t98243;
    let t102546 = -t102534 + t102535 + t96326 + F::new(7.0) / F::new(72.0) * t94485 + t102537 + F::cast_from(0.68598428988911579156e-2_f64) * t98245 + F::cast_from(0.10841600599314203355e-2_f64) * t94498 - F::cast_from(0.22866142996303859718e-3_f64) * t94501 + F::cast_from(0.40015750243531754507e-2_f64) * t94503 + F::cast_from(0.40015750243531754507e-2_f64) * t94505 + F::cast_from(0.10164000561857065645e-3_f64) * t94509 - F::cast_from(0.50820002809285328225e-4_f64) * t94511 - t98253 / F::new(24.0);
    t102546
}
