//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1584/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1584<F: Float>(t33: F, t265: F, t502: F, t18884: F, t20691: F, t21643: F, t1113: F, t1304: F, t1469: F, t1711: F, t18281: F, t1837: F, t18892: F, t20256: F, t4186: F, t4560: F, t504: F, t5509: F, t57: F, t5825: F, t606: F, t6084: F, t6416: F, t6757: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t21645 = piecewise3::<F>(t503, t20691 + t21643, t18884);
    let t21657 = piecewise3::<F>(t400, t18884 * t33 / F::cast_from(2.0_f64) + t6084 * t1113 / F::cast_from(2.0_f64) + t4560 * t1711 - t18892 + t895 * t6416 / F::cast_from(2.0_f64) + t265 * t20256 / F::cast_from(2.0_f64), t21645 * t57 / F::cast_from(2.0_f64) - t6757 * t606 / F::cast_from(2.0_f64) - t5509 * t1469 - t1837 * t4186 - t1304 * t5825 / F::cast_from(2.0_f64) - t504 * t18281 / F::cast_from(2.0_f64));
    t21657
}
