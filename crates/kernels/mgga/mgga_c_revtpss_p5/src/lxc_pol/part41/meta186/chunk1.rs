//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 762/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk762<F: Float>(t30: F, t265: F, t393: F, t4560: F, t5027: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t4186: F, t45: F, t4568: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t5028 = piecewise3::<F>(t394, t5027, t4560);
    let t5035 = piecewise3::<F>(t120, t4560 * t30 / F::new(2.0) + t1587 * t605 / F::new(2.0) + t895 * t1468 / F::new(2.0) + t4568, t1106 * t1469 / F::new(2.0) + t1704 * t606 / F::new(2.0) + t395 * t4186 / F::new(2.0) + t5028 * t45 / F::new(2.0));
    (t5028, t5035)
}
