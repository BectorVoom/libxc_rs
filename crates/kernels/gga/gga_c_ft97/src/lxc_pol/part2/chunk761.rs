//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 761/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk761<F: Float>(t11416: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t11783: F, t11787: F, t11791: F, t11949: F, t8260: F, t11928: F, t11936: F, t11948: F) -> F {
    let t11957 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11416;
    let t11958 = -t11949 - t8260 - t11783 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t11787 - t11791 / F::cast_from(2.0_f64) - t11395 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11399 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t11404 + F::cast_from(2.0_f64) * t11408 + F::cast_from(4.0_f64) * t11413 - t11957;
    let t11960 = t11928 + t11936 + t11948 + t11958;
    t11960
}
