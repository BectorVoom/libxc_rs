//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1971/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1971<F: Float>(t102117: F, t102120: F, t102122: F, t102129: F, t102131: F, t102133: F, t102135: F, t102139: F, t109400: F, t109404: F, t109408: F, t109413: F, t109417: F, t96206: F) -> F {
    let t109423 = F::cast_from(0.25702851531048074406e-1_f64) * t109400 + F::cast_from(0.43368140941025997311e-1_f64) * t109404 + t102117 + F::cast_from(0.72280234901709995518e-2_f64) * t109408 + F::cast_from(0.96373646535613327359e-3_f64) * t102120 - F::cast_from(0.28912093960683998207e-1_f64) * t109413 - F::cast_from(0.26019841438354088051e-1_f64) * t102122 + t96206 + F::cast_from(0.54878743191129263322e-2_f64) * t109417 - t102129 + F::cast_from(0.4818682326780666368e-3_f64) * t102131 + F::cast_from(0.3427046870806409921e-2_f64) * t102133 - F::cast_from(0.45699670022203476294e-2_f64) * t102135 - F::cast_from(0.13009920719177044025e-2_f64) * t102139;
    t109423
}
