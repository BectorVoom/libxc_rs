//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 260/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk260<F: Float>(t30: F, t265: F, t393: F, t1100: F, t1102: F, t198: F, t336: F, t895: F, t912: F, t938: F, t978: F, t980: F, t985: F, t395: F, t45: F, t605: F, t606: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t1106 = piecewise3::<f64>(t394, t1100 * t1102 * t198 * t336 - t912 + t938 + t978 + t980 - t985, t895);
    let t1111 = piecewise3::<f64>(t120, t265 * t605 / F::new(2.0) + t895 * t30 / F::new(2.0), t1106 * t45 / F::new(2.0) + t395 * t606 / F::new(2.0));
    (t1106, t1111)
}
