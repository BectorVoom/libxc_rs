//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1788/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1788<F: Float>(t1940: F, t2071: F, t2255: F, t1468: F, t2403: F, t26425: F, t26585: F, t27160: F, t27166: F, t27169: F, t27173: F, t27376: F, t27385: F, t27387: F, t27391: F, t27395: F, t27402: F, t28291: F, t28456: F, t28460: F, t28472: F, t30: F, t605: F, t7010: F, t7092: F, t7428: F, t7432: F, t7749: F, t7787: F, t8020: F) -> (F, F) {
    let t28490 = t1940 * t2071 * t2255;
    let t28491 = F::cast_from(3.0_f64) * t28291 * t27160 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t7428 * t7749 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t27166 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27169 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27173 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8020 * t7010 + t1940 * t28456 * t30 / F::cast_from(2.0_f64) - t1940 * t28460 * t7092 / F::cast_from(2.0_f64) + t1940 * t8020 * t605 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t26425 * t27376 - t1940 * t26585 * t7787 / F::cast_from(2.0_f64) + t28472 * t27385 - t1940 * t7432 * t27387 / F::cast_from(2.0_f64) - t1940 * t7432 * t27391 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t2071 * t27395 + t1940 * t7428 * t1468 / F::cast_from(2.0_f64) - t1940 * t7432 * t27402 / F::cast_from(2.0_f64) + t28490;
    (t28490, t28491)
}
