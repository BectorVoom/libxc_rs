//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1032/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1032<F: Float>(t30: F, t265: F, t393: F, t127143: F, t127180: F, t127112: F, t126434: F, t1469: F, t32059: F, t33867: F, t4186: F, t45: F, t606: F, t8543: F, t27375: F, t27799: F, t125984: F, t25759: F, t126030: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t127181 = t127143 + t127180;
    let t127182 = piecewise3(t394, t127112, t127181);
    let t127189 = piecewise3(t120, t126434, t127182 * t45 / 2.0 + t32059 * t1469 / 2.0 + t33867 * t606 / 2.0 + t8543 * t4186 / 2.0);
    let t127190 = t27799 * t27375;
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    (t127181, t127189, t127190, t127193, t127199)
}
