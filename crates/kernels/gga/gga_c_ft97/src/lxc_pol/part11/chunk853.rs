//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 853/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk853<F: Float>(t379: F, t9293: F, t446: F, t9073: F, t10: F, t11175: F, t144: F, t2086: F, t590: F, t91: F, t9243: F, t37311: F, t9327: F, t1882: F, t9075: F, t9042: F) -> (F, F, F, F, F, F, F, F) {
    let t39668 = t9293 * t379;
    let t39670 = t446 * t9073 * t39668;
    let t39673 = t10 * t11175 * t144;
    let t39674 = 280.0 / 81.0 * t39673;
    let t39677 = t91 * t2086 * t9243 * t590;
    let t39679 = t446 * t9327 * t37311;
    let t39681 = t1882 * t9075;
    let t39683 = t1882 * t9042;
    (t39668, t39670, t39673, t39674, t39677, t39679, t39681, t39683)
}
