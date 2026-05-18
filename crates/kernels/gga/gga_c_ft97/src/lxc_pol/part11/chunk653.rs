//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 653/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk653<F: Float>(t10: F, t144: F, t3050: F, t1984: F, t378: F, t1986: F, t379: F, t446: F, t1647: F, t558: F, t1969: F, t9039: F, t9043: F, t9047: F, t9052: F, t9057: F, t9059: F, t9062: F, t9065: F, t9068: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9071 = t10 * t3050 * t144;
    let t9072 = F::new(14.0) / F::new(81.0) * t9071;
    let t9073 = t378 * t1984;
    let t9074 = t379 * t1986;
    let t9075 = t9073 * t9074;
    let t9076 = t446 * t9075;
    let t9078 = t1647 * t558;
    let t9079 = t1969 * t9078;
    let t9080 = t446 * t9079;
    let t9082 = -t9039 / F::new(9.0) + t9043 / F::new(6.0) + t9047 / F::new(6.0) + t9052 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t9057 - t9059 / F::new(9.0) - t9062 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t9065 + t9068 / F::new(6.0) - t9072 - t9076 / F::new(3.0) - t9080 / F::new(3.0);
    (t9071, t9073, t9074, t9075, t9076, t9078, t9079, t9080, t9082)
}
