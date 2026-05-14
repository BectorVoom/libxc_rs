//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1350/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1350<F: Float>(t1791: F, t8814: F, t117019: F, t117020: F, t34146: F, t1636: F, t24048: F, t7242: F, t34045: F, t34225: F, t1869: F, t2454: F, t34160: F, t5061: F, t1790: F, t36267: F, t7261: F) -> (F, F, F, F, F, F) {
    let t121038 = t1791 * t8814;
    let t121044 = t117019 * t117020 * t34146;
    let t121052 = t7242 * t24048 * t1636;
    let t121061 = t34225 * t34045;
    let t121067 = t1869 * t5061 * t2454 * t34160;
    let t121071 = t7261 * t36267 * t8814 * t1790;
    (t121038, t121044, t121052, t121061, t121067, t121071)
}
