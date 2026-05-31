//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 916/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk916<F: Float>(t8762: F, t8764: F, t20: F, t4879: F, t62: F, t879: F, t2740: F, t882: F, t209: F, t207: F, t69: F, t2739: F, t6: F) -> (F, F, F, F, F) {
    let t8765 = t8762 * t8764;
    let t8769 = t62 * t4879 * t20;
    let t8778 = t879 * t879;
    let t8779 = F::cast_from(1.0_f64) / t8778;
    let t8780 = t2740 * t882;
    let t8782 = t209 * t8779 * t8780;
    let t8785 = t207 * t69;
    let t8786 = t6 * t2739;
    (t8765, t8769, t8782, t8785, t8786)
}
