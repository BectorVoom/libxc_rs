//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 543/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk543<F: Float>(t2101: F, t3209: F, t1890: F, t723: F, t550: F, t9603: F, t5539: F, t9595: F, t1843: F, t2558: F, t7634: F, t9647: F) -> (F, F, F, F, F, F, F, F) {
    let t9739 = t2101 * t3209;
    let t9740 = t1890 * t723;
    let t9741 = t9739 * t9740;
    let t9744 = t550 * t9603;
    let t9745 = t5539 * t9744;
    let t9748 = t550 * t9595;
    let t9749 = t1843 * t9748;
    let t9752 = t7634 * t2558;
    let t9754 = F::cast_from(0.64087718584518535698e-3_f64) * t9647 * t9752;
    (t9739, t9740, t9741, t9744, t9745, t9748, t9749, t9754)
}
