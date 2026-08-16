//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 793/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk793<F: Float>(t33: F, t265: F, t502: F, t1940: F, t8490: F, t8494: F, t8542: F, t57: F, t1936: F, t6985: F, t8453: F, t93: F, t1312: F, t8460: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8552 = t1940 * t8490 * t33 / F::cast_from(2.0_f64) - t1940 * t8494 * t33 / F::cast_from(2.0_f64);
    let t8553 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t8542);
    let t8556 = piecewise3::<F>(t400, t8552, t8553 * t57 / F::cast_from(2.0_f64));
    let t8559 = t6985 * t1936;
    let t8562 = F::cast_from(2.0_f64) * t93 * t8453;
    let t8563 = t1312 * t8460;
    (t8553, t8556, t8559, t8562, t8563)
}
