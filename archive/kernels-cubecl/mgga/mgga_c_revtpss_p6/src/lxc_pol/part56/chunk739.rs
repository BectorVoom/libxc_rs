//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 739/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk739<F: Float>(t1940: F, t30: F, t8490: F, t8494: F, t207: F, t8489: F, t8493: F, t198: F, t2411: F, t892: F, t33: F, t8453: F, t93: F) -> (F, F, F, F, F, F) {
    let t8498 = t1940 * t8490 * t30 / F::cast_from(2.0_f64) - t1940 * t8494 * t30 / F::cast_from(2.0_f64);
    let t8536 = t207 * t8489;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    let t8552 = t1940 * t8490 * t33 / F::cast_from(2.0_f64) - t1940 * t8494 * t33 / F::cast_from(2.0_f64);
    let t8562 = F::cast_from(2.0_f64) * t93 * t8453;
    (t8498, t8536, t8539, t8542, t8552, t8562)
}
