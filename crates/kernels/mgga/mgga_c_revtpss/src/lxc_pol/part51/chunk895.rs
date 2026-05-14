//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 895/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk895<F: Float>(t30: F, t265: F, t393: F, t1102: F, t1699: F, t198: F, t32030: F, t32036: F, t336: F, t33836: F, t33866: F, t5023: F, t7181: F, t7840: F, t1469: F, t33748: F, t45: F, t8543: F, t33: F, t7782: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t33867 = piecewise3(t394, t1102 * t198 * t336 * t33836 - t1699 * t32030 * t5023 + 2.0 * t1699 * t32036 * t5023 - 2.0 * t5023 * t7181 * t7840, t33866);
    let t33872 = piecewise3(t120, t33748, t8543 * t1469 / 2.0 + t33867 * t45 / 2.0);
    let t33888 = t33 * t7782;
    (t33867, t33872, t33888)
}
