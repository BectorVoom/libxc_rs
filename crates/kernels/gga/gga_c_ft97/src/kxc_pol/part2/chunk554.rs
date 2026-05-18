//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 554/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk554<F: Float>(t3474: F, t3587: F, t160: F, t3539: F, t1023: F, t1058: F, t149: F, t165: F, t3313: F, t3414: F, t3484: F, t3566: F, t3579: F, t3583: F, t564: F, t614: F) -> (F, F, F) {
    let t3588 = t3474 + t3587;
    let t3590 = t3539 * t160;
    let t3596 = -t1023 * t614 - t1058 * t564 - t149 * t3588 - t165 * t3313 - t165 * t3414 + F::new(4.0) * t3484 - F::new(2.0) * t3566 - F::new(2.0) * t3579 - F::new(2.0) * t3583 + F::new(2.0) * t3590;
    (t3588, t3590, t3596)
}
