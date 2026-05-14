//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 859/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk859<F: Float>(t10868: F, t2150: F, t2147: F, t545: F, t775: F, t2206: F, t774: F, t146: F, t2190: F, t261: F, t3299: F, t2218: F, t3304: F, t503: F, t505: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10869 = t10868 * t2150;
    let t10870 = t2147 * t10869;
    let t10872 = t545 * t775;
    let t10878 = t2206 * t774;
    let t10879 = t146 * t10878;
    let t10885 = t261 * t2190;
    let t10886 = t3299 * t10885;
    let t10891 = t261 * t2218;
    let t10892 = t3304 * t10891;
    let t10894 = t503 * t505;
    (t10869, t10870, t10872, t10878, t10879, t10885, t10886, t10891, t10892, t10894)
}
