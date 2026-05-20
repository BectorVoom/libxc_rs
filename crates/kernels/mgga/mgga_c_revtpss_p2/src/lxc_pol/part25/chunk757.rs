//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 757/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk757<F: Float>(t33: F, t265: F, t502: F, t775: F, t890: F, t1113: F, t1940: F, t1963: F, t2403: F, t7087: F, t7091: F, t7193: F, t2003: F, t57: F, t606: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7200 = t33 * t775;
    let t7207 = t33 * t890;
    let t7214 = F::new(3.0) / F::new(2.0) * t2403 * t1963 * t7200 + t1940 * t7087 * t33 / F::new(2.0) - t1940 * t7091 * t7207 / F::new(2.0) + t1940 * t1963 * t1113 / F::new(2.0);
    let t7215 = piecewise3::<F>(t503, F::new(0.0), t7193);
    let t7220 = piecewise3::<F>(t400, t7214, -t2003 * t606 / F::new(2.0) + t7215 * t57 / F::new(2.0));
    (t7200, t7207, t7215, t7220)
}
