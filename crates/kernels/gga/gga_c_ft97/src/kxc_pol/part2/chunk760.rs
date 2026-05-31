//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 760/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk760<F: Float>(t11069: F, t11041: F, t11048: F, t11052: F, t11056: F, t11061: F, t11066: F, t11073: F, t11774: F, t11939: F, t8454: F, t11076: F) -> (F, F) {
    let t11946 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11069;
    let t11948 = -F::cast_from(6.0_f64) * t11041 - t11939 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11048 - F::cast_from(2.0_f64) * t11052 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11056 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11061 + t11774 / F::cast_from(2.0_f64) - t8454 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11066 + t11946 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11073;
    let t11949 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11076;
    (t11948, t11949)
}
