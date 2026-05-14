//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 945/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk945<F: Float>(t1742: F, t3188: F, t5570: F, t53: F, t920: F, t423: F, t22515: F, t1737: F, t3057: F, t5571: F, t3099: F, t5546: F, t5790: F, t938: F, t1294: F, t1602: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25709 = t1742 * t3188;
    let t25710 = t5570 * t25709;
    let t25713 = t920 * t53;
    let t25714 = t423 * t25713;
    let t25715 = t22515 * t25714;
    let t25718 = t1737 * t3188;
    let t25719 = t5570 * t25718;
    let t25722 = t5571 * t3057;
    let t25726 = t5546 * t3099;
    let t25730 = t5790 * t938;
    let t25734 = t1602 * t1294;
    (t25709, t25710, t25713, t25714, t25715, t25718, t25719, t25722, t25726, t25730, t25734)
}
