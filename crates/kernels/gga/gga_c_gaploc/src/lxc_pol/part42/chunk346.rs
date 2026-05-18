//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 346/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk346<F: Float>(t524: F, t999: F, t189: F, t2754: F, t188: F, t2792: F, t531: F, t569: F, t568: F, t1457: F, t2779: F, t2778: F, t475: F) -> (F, F, F, F, F, F, F) {
    let t2819 = t524 * t999;
    let t2822 = t189 * t2754;
    let t2823 = t188 * t2822;
    let t2828 = t531 * t2792;
    let t2833 = t569 * t2754;
    let t2834 = t568 * t2833;
    let t2843 = t1457 * t2779;
    let t2846 = t2778 * t475;
    (t2819, t2822, t2823, t2828, t2834, t2843, t2846)
}
