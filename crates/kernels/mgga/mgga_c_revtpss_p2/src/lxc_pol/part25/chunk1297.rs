//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1297/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1297<F: Float>(t30: F, t10326: F, t1996: F, t2258: F, t25744: F, t45: F, t606: F, t7194: F, t93409: F, t94214: F, t25759: F, t51806: F, t27799: F, t50066: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t94224 = piecewise3::<F>(t120, t93409, t94214 * t45 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t25744 * t606 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t7194 * t2258 + t1996 * t10326 / F::cast_from(2.0_f64));
    let t94228 = t25759 * t51806;
    let t94231 = t27799 * t50066;
    (t94224, t94228, t94231)
}
