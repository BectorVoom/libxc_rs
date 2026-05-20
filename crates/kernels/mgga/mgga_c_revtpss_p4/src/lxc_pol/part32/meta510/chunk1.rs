//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1802/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802<F: Float>(t2061: F, t6071: F, t7071: F, t26462: F, t26468: F, t26471: F, t27228: F, t27230: F, t27256: F, t29623: F, t29627: F, t29629: F, t29631: F, t29633: F) -> (F, F, F) {
    let t30356 = t2061 * t6071;
    let t30357 = t7071 * t30356;
    let t30378 = t26462 + t29623 / F::new(8.0) - F::cast_from(0.10164000561857065645e-3_f64) * t27228 + F::cast_from(0.80031500487063509014e-2_f64) * t27230 + F::cast_from(0.17149607247227894789e-1_f64) * t29627 - t29629 / F::new(24.0) + F::cast_from(0.32012600194825403606e-1_f64) * t27256 + t26468 - t26471 - F::cast_from(0.85748036236139473944e-3_f64) * t29631 - F::cast_from(0.34299214494455789578e-2_f64) * t29633;
    (t30356, t30357, t30378)
}
