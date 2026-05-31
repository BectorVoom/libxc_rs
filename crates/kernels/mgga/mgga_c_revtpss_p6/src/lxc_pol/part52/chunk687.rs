//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 687/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk687<F: Float>(t33: F, t265: F, t502: F, t1113: F, t1940: F, t1963: F, t2403: F, t7087: F, t7091: F, t7200: F, t7207: F, t7193: F, t2003: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7214 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t7200 + t1940 * t7087 * t33 / F::cast_from(2.0_f64) - t1940 * t7091 * t7207 / F::cast_from(2.0_f64) + t1940 * t1963 * t1113 / F::cast_from(2.0_f64);
    let t7215 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t7193);
    let t7220 = piecewise3::<F>(t400, t7214, -t2003 * t606 / F::cast_from(2.0_f64) + t7215 * t57 / F::cast_from(2.0_f64));
    (t7215, t7220)
}
