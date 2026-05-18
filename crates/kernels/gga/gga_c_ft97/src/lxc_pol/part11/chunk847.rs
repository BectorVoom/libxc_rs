//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 847/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk847<F: Float>(t356: F, t359: F, t37391: F, t89: F, t1588: F, t1755: F, t28: F, t7241: F, t375: F, t7760: F, t7766: F, t1556: F, t1569: F) -> (F, F, F, F, F) {
    let t37394 = t89 * t356 * t359 * t37391;
    let t37399 = t89 * t28 * t7241 * t1588 * t1755;
    let t37401 = t375 * t7760;
    let t37403 = t89 * t37401 * t7766;
    let t37406 = F::new(1.0) / t1556 / t1569;
    (t37394, t37399, t37401, t37403, t37406)
}
