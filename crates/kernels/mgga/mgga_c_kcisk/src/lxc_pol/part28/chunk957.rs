//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 957/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk957<F: Float>(t22534: F, t4629: F, t1648: F, t8504: F, t2372: F, t6771: F, t8522: F, t821: F, t8511: F) -> (F, F, F, F, F) {
    let t22535 = t4629 * t22534;
    let t22542 = t8504 * t1648;
    let t22547 = t2372 * t6771;
    let t22556 = t8522 * t1648;
    let t22564 = t821 * t8511;
    (t22535, t22542, t22547, t22556, t22564)
}
