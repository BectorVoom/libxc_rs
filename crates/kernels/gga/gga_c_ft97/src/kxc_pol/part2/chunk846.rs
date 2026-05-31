//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 846/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk846<F: Float>(t1023: F, t1058: F, t12278: F, t12597: F, t12600: F, t12603: F, t12606: F, t13245: F, t165: F, t1953: F, t2081: F, t2228: F, t3414: F, t3588: F, t564: F, t614: F) -> F {
    let t13246 = -t1023 * t2228 - t1058 * t1953 - t1058 * t2081 - t12597 * t165 - F::cast_from(2.0_f64) * t3414 * t614 - F::cast_from(2.0_f64) * t3588 * t564 - F::cast_from(4.0_f64) * t12278 + F::cast_from(4.0_f64) * t12600 + F::cast_from(8.0_f64) * t12603 - F::cast_from(12.0_f64) * t12606 + t13245;
    t13246
}
