//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1942/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1942<F: Float>(t1665: F, t1671: F, t25500: F, t25505: F, t25509: F, t25517: F, t25522: F, t25560: F, t25580: F, t27450: F, t27479: F, t27539: F, t6263: F, t6268: F, t6273: F, t6278: F, t6302: F, t6308: F, t6312: F, t6331: F, t6339: F, t7117: F, t7122: F, t7132: F) -> F {
    let t29806 = -F::cast_from(0.57165357490759649296e-3_f64) * t7132 * t6331 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t6268 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t6302 + F::cast_from(0.85748036236139473944e-3_f64) * t25505 * t6308 - F::cast_from(0.42874018118069736972e-3_f64) * t25509 * t6312 + F::cast_from(0.85748036236139473944e-3_f64) * t27450 * t1671 - F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t6263 + F::cast_from(0.85748036236139473944e-3_f64) * t25500 * t6339 - F::cast_from(0.85748036236139473944e-3_f64) * t27479 * t1665 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t6278 - t25560 + F::cast_from(0.3811023832717309953e-3_f64) * t27539 - F::cast_from(0.85748036236139473944e-3_f64) * t25580 * t6273;
    t29806
}
