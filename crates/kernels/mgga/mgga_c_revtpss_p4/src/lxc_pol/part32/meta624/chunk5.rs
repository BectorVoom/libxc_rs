//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1973/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1973<F: Float>(t30308: F, t686: F, t72: F, t25895: F, t25878: F, t109425: F, t25899: F, t30261: F, t689: F, t25904: F, t102205: F, t102213: F, t102217: F, t102219: F, t102225: F, t102237: F, t102239: F, t27837: F, t28841: F, t96246: F, t96253: F) -> F {
    let t109449 = t30308 * t72 * t686;
    let t109450 = t25895 * t109449;
    let t109453 = t25878 * t109449;
    let t109455 = t25899 * t109425;
    let t109457 = t30261 * t689;
    let t109458 = t25904 * t109457;
    let t109460 = t25899 * t109457;
    let t109467 = F::cast_from(0.91399340044406952588e-2_f64) * t102205 - F::cast_from(0.14456046980341999104e-1_f64) * t109450 + t102213 - t102217 + F::cast_from(0.38549458614245330944e-1_f64) * t102219 + F::cast_from(0.25702851531048074406e-1_f64) * t109453 + F::cast_from(0.12851425765524037203e-1_f64) * t109455 - F::cast_from(0.72280234901709995518e-2_f64) * t109458 + F::cast_from(0.12851425765524037203e-1_f64) * t109460 - F::cast_from(0.68540937416128198419e-2_f64) * t102225 - F::cast_from(0.17135234354032049604e-1_f64) * t96246 - F::cast_from(0.65049603595885220126e-3_f64) * t96253 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t28841 + t102237 - t102239;
    t109467
}
