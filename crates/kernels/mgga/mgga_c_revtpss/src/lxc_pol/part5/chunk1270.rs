//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1270/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1270<F: Float>(t30: F, t265: F, t393: F, t18884: F, t19141: F, t20234: F, t1106: F, t1468: F, t1469: F, t1704: F, t18280: F, t18281: F, t18892: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t5824: F, t5825: F, t605: F, t606: F, t6084: F, t6405: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t20236 = piecewise3::<F>(t394, t19141 + t20234, t18884);
    let t20248 = piecewise3::<F>(t120, t18884 * t30 / F::new(2.0) + t6084 * t605 / F::new(2.0) + t4560 * t1468 + t18892 + t895 * t5824 / F::new(2.0) + t265 * t18280 / F::new(2.0), t20236 * t45 / F::new(2.0) + t6405 * t606 / F::new(2.0) + t5028 * t1469 + t1704 * t4186 + t1106 * t5825 / F::new(2.0) + t395 * t18281 / F::new(2.0));
    t20248
}
