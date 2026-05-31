//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2033/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2033<F: Float>(t110698: F, t892: F, t198: F, t205: F, t8019: F, t102854: F, t105906: F, t106534: F, t106540: F, t106546: F, t106562: F, t106590: F, t106593: F, t106606: F, t1468: F, t1940: F, t26425: F, t26585: F, t26590: F, t27160: F, t28291: F, t28456: F, t28472: F, t29599: F, t29719: F, t30: F, t7432: F, t7787: F, t95511: F) -> (F, F, F) {
    let t110699 = t110698 * t892;
    let t110704 = t198 * t205 * t8019;
    let t110711 = -t1940 * t26585 * t29719 / F::cast_from(2.0_f64) - t1940 * t7432 * t106606 / F::cast_from(2.0_f64) - F::cast_from(6.0_f64) * t28291 * t106534 - F::cast_from(3.0_f64) * t26425 * t105906 + t1940 * t26590 * t106593 - t1940 * t102854 * t7787 + F::cast_from(3.0_f64) * t26425 * t106562 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t106540 + F::cast_from(2.0_f64) * t28472 * t106590 + F::cast_from(6.0_f64) * t28291 * t106546 + t1940 * t110699 * t30 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t110704 * t27160 + t1940 * t28456 * t1468 - F::cast_from(3.0_f64) * t95511 * t29599;
    (t110699, t110704, t110711)
}
