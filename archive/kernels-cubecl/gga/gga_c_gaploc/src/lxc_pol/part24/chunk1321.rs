//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1321/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1321<F: Float>(t33155: F, t6066: F, t7630: F, t10860: F, t23279: F, t33627: F, t2033: F, t2365: F, t2610: F, t8720: F, t15349: F, t3474: F) -> (F, F, F, F, F) {
    let t33640 = F::cast_from(0.71500979903700853338e0_f64) * t7630 * t6066 * t33155;
    let t33642 = F::cast_from(0.14300195980740170668e1_f64) * t23279 * t10860;
    let t33645 = F::cast_from(0.14300195980740170668e1_f64) * t7630 * t6066 * t33627;
    let t33648 = t2033 * t2365 * t2610 * t8720;
    let t33649 = F::cast_from(0.14896037479937677779e-1_f64) * t33648;
    let t33650 = t15349 * t3474;
    (t33640, t33642, t33645, t33649, t33650)
}
