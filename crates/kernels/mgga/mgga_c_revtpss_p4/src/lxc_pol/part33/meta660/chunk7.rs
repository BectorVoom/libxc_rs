//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2144/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2144<F: Float>(t29598: F, t775: F, t25207: F, t1940: F, t2255: F, t7783: F, t77425: F, t106498: F, t106502: F, t106510: F, t106516: F, t106520: F, t106528: F, t1468: F, t2403: F, t25206: F, t27158: F, t27166: F, t27173: F, t27364: F, t27368: F, t27391: F, t29705: F, t605: F, t7091: F, t7092: F, t7787: F, t98637: F, t99555: F) -> (F, F, F) {
    let t106533 = t29598 * t775;
    let t106534 = t25207 * t106533;
    let t106539 = F::cast_from(2.0_f64) * t1940 * t7783 * t2255;
    let t106540 = t25207 * t77425;
    let t106543 = F::cast_from(3.0_f64) * t27158 * t106498 + F::cast_from(6.0_f64) * t25206 * t106502 + F::cast_from(3.0_f64) * t2403 * t7783 * t27173 + t1940 * t27364 * t1468 - t1940 * t7091 * t106510 / F::cast_from(2.0_f64) - t1940 * t99555 * t7787 - t1940 * t106516 * t7092 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t25206 * t106520 - F::cast_from(3.0_f64) * t98637 * t27166 + t1940 * t29705 * t605 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t25206 * t106528 - t1940 * t27368 * t27391 - F::cast_from(6.0_f64) * t27158 * t106534 + t106539 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t25206 * t106540;
    (t106533, t106539, t106543)
}
