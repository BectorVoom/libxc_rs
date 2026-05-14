//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 754/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk754<F: Float>(t2086: F, t3526: F, t590: F, t91: F, t2120: F, t3491: F, t12574: F, t12577: F, t12580: F, t12584: F, t12589: F, t12592: F, t12918: F, t12921: F, t9062: F, t12893: F, t12905: F, t12917: F) -> (F, F, F) {
    let t12923 = t2086 * t3526;
    let t12925 = t91 * t12923 * t590;
    let t12928 = t91 * t3491 * t2120;
    let t12937 = -t12918 + t12921 / 8.0 - t12925 / 6.0 - t12928 / 12.0 + 2.0 / 9.0 * t12574 + 8.0 / 9.0 * t12577 - 2.0 / 27.0 * t12580 + 2.0 / 3.0 * t12584 - 2.0 * t12589 + 4.0 / 27.0 * t12592 - 2.0 / 27.0 * t9062;
    let t12939 = t12893 + t12905 + t12917 + t12937;
    (t12925, t12928, t12939)
}
