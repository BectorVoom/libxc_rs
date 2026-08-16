//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 985/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk985<F: Float>(t33: F, t265: F, t502: F, t28522: F, t1469: F, t2085: F, t28577: F, t4186: F, t57: F, t606: F, t7468: F, t8059: F, t28530: F, t26405: F, t27153: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t28578 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t28522);
    let t28585 = piecewise3::<F>(t400, t28577, -t7468 * t1469 / F::cast_from(2.0_f64) - t2085 * t4186 / F::cast_from(2.0_f64) + t28578 * t57 / F::cast_from(2.0_f64) - t8059 * t606 / F::cast_from(2.0_f64));
    let t28586 = t28530 + t28585;
    let t28588 = t26405 * t27153;
    (t28586, t28588)
}
