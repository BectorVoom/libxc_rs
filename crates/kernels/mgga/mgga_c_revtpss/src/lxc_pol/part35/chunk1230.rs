//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1230/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1230<F: Float>(t33: F, t265: F, t502: F, t115819: F, t115870: F, t115913: F, t1469: F, t2085: F, t22671: F, t30503: F, t57: F, t5825: F, t8059: F, t114378: F, t114452: F, t114800: F, t114812: F, t114820: F, t114905: F, t115358: F, t115830: F, t118: F, t1502: F, t18245: F, t1843: F, t2014: F, t2056: F, t22483: F, t25082: F, t26405: F, t28167: F, t29508: F, t30138: F, t30315: F, t30511: F, t30558: F, t30578: F, t30584: F, t30589: F, t4248: F, t508: F, t651: F, t7732: F, t7898: F, t7978: F, t7984: F, t7988: F, t8108: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t115915 = piecewise3::<f64>(t503, F::new(0.0), t115819);
    let t115925 = piecewise3::<f64>(t400, t115870 + t115913, t115915 * t57 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t30503 * t1469 - F::new(3.0) / F::new(2.0) * t8059 * t5825 - t2085 * t22671 / F::new(2.0));
    let t115962 = -F::new(6.0) * t18245 * t7984 - F::new(6.0) * t114378 * t2056 - F::new(12.0) * t30138 * t7978 - F::new(9.0) * t25082 * t26405 * t114800 - t118 * (t115830 + t115925) - F::new(3.0) * t1502 * t30511 - F::new(18.0) * t28167 * t26405 * t114452 - F::new(9.0) * t25082 * t26405 * t114820 - F::new(3.0) * t7898 * t30584 + F::new(3.0) * t7898 * t30315 - F::new(3.0) * t2014 * t8108 * t22483 - F::new(2.0) * t114812 * t2056 - F::new(6.0) * t29508 * t7978 - F::new(2.0) * t651 * t508 * t114905 - F::new(6.0) * t18245 * t7988 - F::new(12.0) * t4248 * t30578 - F::new(6.0) * t115358 * t508 - F::new(6.0) * t30589 * t1843 - F::new(6.0) * t4248 * t30558 - F::new(6.0) * t7732 * t30558;
    t115962
}
