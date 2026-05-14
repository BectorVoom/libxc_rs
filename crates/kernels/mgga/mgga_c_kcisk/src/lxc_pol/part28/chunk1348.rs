//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1348/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1348<F: Float>(t11694: F, t35277: F, t117580: F, t7296: F, t23868: F, t7261: F, t9650: F, t1869: F, t1894: F, t34159: F, t9029: F, t1757: F, t33017: F, t71399: F, t17182: F, t35135: F) -> (F, F, F, F, F, F) {
    let t121006 = 4.0 * t11694 * t35277;
    let t121008 = 4.0 * t117580 * t7296;
    let t121010 = t7261 * t9650 * t23868;
    let t121015 = t1869 * t34159 * t9029 * t1894;
    let t121019 = t1869 * t33017 * t71399 * t1757;
    let t121021 = t17182 * t35135;
    (t121006, t121008, t121010, t121015, t121019, t121021)
}
