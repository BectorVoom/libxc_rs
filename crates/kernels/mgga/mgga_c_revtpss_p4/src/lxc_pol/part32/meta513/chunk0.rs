//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1809/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1809<F: Float>(t30: F, t265: F, t393: F, t30462: F, t1469: F, t2078: F, t30438: F, t45: F, t5825: F, t8040: F, t2071: F, t29939: F, t1711: F, t1940: F, t2403: F, t26425: F, t26590: F, t28460: F, t29946: F, t29949: F, t29953: F, t29964: F, t29967: F, t29970: F, t30420: F, t33: F, t4541: F, t6416: F, t7432: F, t7862: F, t7869: F, t8020: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t30463 = piecewise3::<F>(t394, F::new(0.0), t30462);
    let t30470 = piecewise3::<F>(t120, t30438, t30463 * t45 / F::new(2.0) + t8040 * t1469 + t2078 * t5825 / F::new(2.0));
    let t30471 = t2071 * t29939;
    let t30502 = F::new(3.0) * t4541 * t30471 + F::new(3.0) * t2403 * t8020 * t7862 - F::new(3.0) * t26425 * t29946 + F::new(3.0) * t2403 * t2071 * t29949 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t29953 + t1940 * t30420 * t33 / F::new(2.0) - t1940 * t28460 * t7869 + t1940 * t8020 * t1711 + t1940 * t26590 * t29964 - t1940 * t7432 * t29967 - t1940 * t7432 * t29970 / F::new(2.0) + t1940 * t2071 * t6416 / F::new(2.0);
    (t30463, t30470, t30471, t30502)
}
