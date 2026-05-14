//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1040/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1040<F: Float>(t10491: F, t6318: F, t1491: F, t2999: F, t89: F, t1636: F, t6343: F, t25155: F, t375: F, t24969: F, t681: F, t24983: F, t99312: F, t10570: F, t683: F, t24990: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99529 = t10491 * t6318;
    let t99534 = t89 * t2999 * t1491;
    let t99535 = 28.0 / 27.0 * t99534;
    let t99537 = t89 * t1636 * t6343;
    let t99545 = t89 * t375 * t25155;
    let t99555 = t89 * t681 * t24969;
    let t99557 = t99312 * t24983;
    let t99559 = t683 * t10570;
    let t99567 = t89 * t681 * t24990;
    (t99529, t99534, t99535, t99537, t99545, t99555, t99557, t99559, t99567)
}
