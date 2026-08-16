//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1301/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1301<F: Float>(t33: F, t265: F, t502: F, t94272: F, t94324: F, t94213: F, t10326: F, t2003: F, t2258: F, t25792: F, t57: F, t606: F, t7215: F, t25082: F, t49630: F, t8717: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t94325 = t94272 + t94324;
    let t94326 = piecewise3::<F>(t503, F::cast_from(0.0_f64), t94213);
    let t94336 = piecewise3::<F>(t400, t94325, t94326 * t57 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t25792 * t606 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t7215 * t2258 - t2003 * t10326 / F::cast_from(2.0_f64));
    let t94341 = F::cast_from(9.0_f64) * t25082 * t8717 * t49630;
    (t94336, t94341)
}
