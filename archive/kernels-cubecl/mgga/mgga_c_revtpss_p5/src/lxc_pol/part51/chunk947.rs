//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 947/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk947<F: Float>(t30: F, t265: F, t393: F, t1100: F, t1102: F, t198: F, t32026: F, t32030: F, t32036: F, t32058: F, t336: F, t5023: F, t7177: F, t7181: F, t31882: F, t45: F, t606: F, t8543: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t32059 = piecewise3::<F>(t394, t1102 * t198 * t32026 * t336 - t1100 * t32030 * t5023 + F::cast_from(2.0_f64) * t1100 * t32036 * t5023 - F::cast_from(2.0_f64) * t5023 * t7177 * t7181, t32058);
    let t32064 = piecewise3::<F>(t120, t31882, t32059 * t45 / F::cast_from(2.0_f64) + t8543 * t606 / F::cast_from(2.0_f64));
    (t32059, t32064)
}
