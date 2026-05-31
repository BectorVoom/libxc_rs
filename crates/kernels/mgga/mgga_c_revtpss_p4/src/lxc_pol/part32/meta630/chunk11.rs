//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2042/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2042<F: Float>(t33: F, t265: F, t502: F, t110840: F, t110883: F, t110920: F, t110954: F, t110989: F, t1469: F, t18281: F, t2085: F, t28578: F, t30503: F, t4186: F, t57: F, t5825: F, t606: F, t7468: F, t8059: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t110992 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t110840);
    let t111004 = piecewise3::<F>(t400, t110883 + t110920 + t110954 + t110989, t110992 * t57 / F::cast_from(2.0_f64) - t30503 * t606 / F::cast_from(2.0_f64) - t28578 * t1469 - t8059 * t4186 - t7468 * t5825 / F::cast_from(2.0_f64) - t2085 * t18281 / F::cast_from(2.0_f64));
    t111004
}
