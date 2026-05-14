//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1215/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1215<F: Float>(t1136: F, t15722: F, t14293: F, t2240: F, t4169: F, t6239: F, t382: F, t5967: F, t140: F, t3529: F, t5598: F, t13436: F, t2110: F, t164: F, t398: F, t3929: F, t5798: F) -> (F, F, F, F, F, F, F, F) {
    let t44181 = t1136 * t15722;
    let t48680 = t2240 * t14293;
    let t48691 = t6239 * t4169;
    let t51845 = t382 * t5967;
    let t52483 = t140 * t5598 * t3529;
    let t52538 = t2110 * t13436;
    let t53214 = t164 * t398;
    let t54621 = t5798 * t3929;
    (t44181, t48680, t48691, t51845, t52483, t52538, t53214, t54621)
}
