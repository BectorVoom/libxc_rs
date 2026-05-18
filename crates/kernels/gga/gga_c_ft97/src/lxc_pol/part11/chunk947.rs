//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 947/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk947<F: Float>(t39673: F, t2086: F, t590: F, t91: F, t9243: F, t37311: F, t446: F, t9327: F, t1882: F, t9075: F, t9042: F, t9034: F) -> (F, F, F, F, F, F) {
    let t39674 = F::new(280.0) / F::new(81.0) * t39673;
    let t39677 = t91 * t2086 * t9243 * t590;
    let t39679 = t446 * t9327 * t37311;
    let t39681 = t1882 * t9075;
    let t39683 = t1882 * t9042;
    let t39685 = t1882 * t9034;
    (t39674, t39677, t39679, t39681, t39683, t39685)
}
