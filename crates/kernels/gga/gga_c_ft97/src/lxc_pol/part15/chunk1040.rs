//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1040/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1040<F: Float>(t86354: F, t86370: F, t86386: F, t86402: F, t16246: F, t4589: F, t103: F, t11902: F, t11906: F, t15901: F, t16030: F, t1901: F, t20225: F, t20229: F, t20287: F, t28: F, t39317: F, t446: F, t4611: F, t47926: F, t60984: F, t75766: F, t75845: F, t82: F, t83: F, t8557: F, t89: F) -> (F, F, F) {
    let t86404 = t86354 + t86370 + t86386 + t86402;
    let t86411 = t16246 * t4589;
    let t86422 = -F::new(4.0) / F::new(3.0) * t1901 * t8557 * t15901 * t4611 - F::new(8.0) / F::new(3.0) * t75766 - F::new(16.0) / F::new(27.0) * t60984 + F::new(4.0) / F::new(3.0) * t1901 * t11902 * t20287 + t89 * t28 * t82 * t86404 * t103 / F::new(3.0) + F::new(112.0) / F::new(243.0) * t47926 + t39317 - F::new(2.0) * t446 * t83 * t86411 - F::new(4.0) / F::new(9.0) * t75845 + F::new(8.0) / F::new(3.0) * t1901 * t11906 * t20225 - F::new(8.0) / F::new(9.0) * t1901 * t16030 * t20229;
    (t86404, t86411, t86422)
}
