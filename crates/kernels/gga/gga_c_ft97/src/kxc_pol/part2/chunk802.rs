//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 802/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk802<F: Float>(t1170: F, t8232: F, t3953: F, t681: F, t89: F, t2469: F, t3859: F, t729: F, t3972: F, t713: F, t762: F, t10048: F, t10062: F, t10064: F, t10090: F, t14212: F, t14215: F, t14219: F, t14223: F, t14224: F, t14228: F, t14232: F, t3281: F, t446: F) -> (F,) {
    let t14233 = t8232 * t1170;
    let t14240 = 2.0 / 9.0 * t89 * t681 * t3953;
    let t14242 = t729 * t2469 * t3859;
    let t14245 = t3972 * t713;
    let t14247 = t729 * t762 * t14245;
    let t14251 = t14212 + 2.0 / 3.0 * t446 * t14215 + 4.0 / 9.0 * t3281 * t14219 - t14223 - 4.0 / 81.0 * t14224 - 2.0 / 3.0 * t446 * t14228 + t14232 - 4.0 / 27.0 * t14233 + t10048 / 9.0 - 2.0 / 9.0 * t10062 - 2.0 / 9.0 * t10064 - t14240 + 2.0 / 3.0 * t446 * t14242 + 2.0 / 3.0 * t446 * t14247 - 2.0 / 27.0 * t10090;
    (t14251,)
}
