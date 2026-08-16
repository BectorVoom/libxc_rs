//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 980/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk980<F: Float>(t179: F, t7350: F, t780: F, t1066: F, t5672: F, t299: F, t2939: F, t771: F, t2068: F, t2739: F, t655: F, t759: F) -> (F, F, F, F, F, F, F) {
    let t7751 = t179 * t780 * t7350;
    let t7755 = t179 * t5672 * t1066;
    let t7756 = t299 * t7755;
    let t7760 = t771 * t2939;
    let t7765 = t179 * t2068 * t2739;
    let t7767 = F::cast_from(0.57165357490759649296e-3_f64) * t299 * t7765;
    let t7768 = t759 * t655;
    (t7751, t7755, t7756, t7760, t7765, t7767, t7768)
}
