//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1353/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1353<F: Float>(t1799: F, t24068: F, t9679: F, t1693: F, t1772: F, t8792: F, t34107: F, t6662: F, t22984: F, t33017: F, t112192: F, t2364: F, t34180: F, t116914: F, t2469: F, t4597: F, t6667: F) -> (F, F, F, F, F, F) {
    let t121109 = t1799 * t9679 * t24068;
    let t121116 = t1693 * t8792 * t1772;
    let t121124 = t1799 * t34107 * t6662;
    let t121127 = t1799 * t33017 * t22984;
    let t121133 = t112192 * t2364 * t34180;
    let t121140 = t116914 * t2469 * t4597 * t6667;
    (t121109, t121116, t121124, t121127, t121133, t121140)
}
