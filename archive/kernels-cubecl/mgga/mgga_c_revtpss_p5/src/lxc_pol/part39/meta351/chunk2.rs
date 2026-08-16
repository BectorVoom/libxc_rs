//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1204/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1204<F: Float>(t10574: F, t10566: F, t10568: F, t11075: F, t14340: F, t14343: F, t14345: F, t14352: F, t14353: F, t14364: F, t14365: F, t14372: F, t14373: F, t14374: F, t14375: F, t14379: F, t1544: F, t1940: F, t198: F, t2403: F, t2404: F, t2430: F, t2832: F, t4343: F, t4546: F, t4556: F, t775: F, t9394: F) -> (F, F) {
    let t14380 = F::cast_from(0.18311447306006545054e-3_f64) * t10574;
    let t14381 = F::cast_from(3.0_f64) * t11075 * t1544 * t2403 + F::cast_from(6.0_f64) * t14353 * t2403 * t775 - F::cast_from(6.0_f64) * t14365 * t2403 * t4556 + F::cast_from(6.0_f64) * t14375 * t1544 * t198 - t1940 * t2832 * t4556 + F::cast_from(6.0_f64) * t2403 * t2404 * t4343 + F::cast_from(3.0_f64) * t2403 * t2430 * t4546 + t10566 - t10568 + t14340 + t14343 + t14345 + t14352 + t14364 + t14372 + t14373 + t14374 + t14379 - t14380 + t9394;
    (t14380, t14381)
}
