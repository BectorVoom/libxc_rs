//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1820/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820<F: Float>(t1017: F, t1028: F, t1047: F, t25490: F, t25495: F, t25498: F, t25500: F, t25505: F, t25509: F, t25512: F, t25517: F, t25522: F, t25526: F, t25529: F, t25532: F, t25535: F, t25538: F, t25539: F, t3097: F, t3130: F, t3136: F, t3157: F, t3164: F, t3208: F, t3220: F, t348: F, t7117: F, t7122: F) -> F {
    let t25542 = -F::cast_from(0.85748036236139473944e-3_f64) * t25490 * t1028 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t3220 + F::cast_from(0.45732285992607719436e-2_f64) * t25495 * t1028 - F::cast_from(0.57165357490759649296e-3_f64) * t25498 + F::cast_from(0.85748036236139473944e-3_f64) * t25500 * t3208 + F::cast_from(0.85748036236139473944e-3_f64) * t25505 * t3157 - F::cast_from(0.42874018118069736972e-3_f64) * t25509 * t3164 + F::cast_from(0.85748036236139473944e-3_f64) * t25512 * t1047 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t3097 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t3136 - F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t3130 - F::cast_from(0.45732285992607719436e-2_f64) * t25526 * t1047 + F::cast_from(0.57165357490759649296e-3_f64) * t25529 + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t25532 * t348 - t25535 / F::cast_from(54.0_f64) - t25538 - t25539 * t1017 / F::cast_from(54.0_f64);
    t25542
}
