//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1223/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1223<F: Float>(t2666: F, t6448: F, t19890: F, t6093: F, t7619: F, t2115: F, t6359: F, t545: F, t7600: F, t146: F, t6091: F, t978: F, t2147: F, t6398: F, t8066: F, t6395: F, t8153: F) -> (F, F, F, F, F, F, F) {
    let t26251 = t6448 * t2666;
    let t26258 = t6093 * t19890 * t7619;
    let t26259 = 0.6112917064160653851e0 * t26258;
    let t26260 = t2115 * t6359;
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t26294 = t2147 * t6398 * t8066;
    let t26295 = 0.2037639021386884617e0 * t26294;
    let t26296 = t6395 * t8153;
    (t26251, t26259, t26260, t26278, t26282, t26295, t26296)
}
