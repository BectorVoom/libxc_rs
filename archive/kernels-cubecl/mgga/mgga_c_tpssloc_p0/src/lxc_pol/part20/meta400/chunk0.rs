//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1796/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1796<F: Float>(t2940: F, t4498: F, t2925: F, t4488: F, t959: F, t1634: F, t3175: F, t10165: F, t1065: F, t4693: F, t3174: F, t2970: F, t4343: F) -> (F, F, F, F, F, F) {
    let t13731 = F::cast_from(0.34631718211362927518e2_f64) * t2940 * t4498;
    let t13732 = t4488 * t2925;
    let t13734 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t13732;
    let t13735 = t1634 * t3175;
    let t13736 = t10165 * t13735;
    let t13742 = t4693 * t1065;
    let t13743 = t3174 * t13742;
    let t13748 = t2970 * t4343;
    (t13731, t13732, t13734, t13736, t13743, t13748)
}
