//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 775/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk775<F: Float>(t30: F, t265: F, t393: F, t1100: F, t1102: F, t1699: F, t198: F, t25709: F, t25713: F, t27708: F, t27712: F, t27717: F, t27754: F, t336: F, t5019: F, t5023: F, t7181: F, t1469: F, t1996: F, t27408: F, t4186: F, t45: F, t606: F, t7194: F, t7856: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F,) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t27755 = piecewise3(t394, t1102 * t198 * t27708 * t336 - t1100 * t27712 * t5023 - t1699 * t25709 * t5023 + 2.0 * t25713 * t27717 * t5023 - t5019 * t5023 * t7181, t27754);
    let t27762 = piecewise3(t120, t27408, t7194 * t1469 / 2.0 + t1996 * t4186 / 2.0 + t27755 * t45 / 2.0 + t7856 * t606 / 2.0);
    (t27762,)
}
