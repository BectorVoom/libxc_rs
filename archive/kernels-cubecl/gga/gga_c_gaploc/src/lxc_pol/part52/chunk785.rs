//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 785/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk785<F: Float>(t3177: F, t8272: F, t9267: F, t12953: F, t4781: F, t10318: F, t1397: F, t9287: F, t2487: F, t2754: F, t9438: F, t9448: F) -> (F, F, F, F) {
    let t41903 = t9267 * t8272 * t3177;
    let t41906 = t4781 * t12953;
    let t41914 = t1397 * t10318 * t9287;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    (t41903, t41906, t41914, t41918)
}
