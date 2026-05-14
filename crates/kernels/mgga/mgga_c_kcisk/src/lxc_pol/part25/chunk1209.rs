//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1209/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1209<F: Float>(t4822: F, t6884: F, t16562: F, t1692: F, t5531: F, t7654: F, t11700: F, t2541: F, t12351: F, t2656: F, t1785: F, t220: F, t2023: F, t7638: F, t2642: F, t5509: F) -> (F, F, F, F, F, F, F, F) {
    let t64908 = t6884 * t4822;
    let t64926 = t16562 * t1692;
    let t64998 = t7654 * t5531;
    let t65005 = t2541 * t11700;
    let t65015 = t2656 * t12351;
    let t68280 = t1785 * t220;
    let t74445 = t2023 * t7638;
    let t74475 = t2642 * t5509;
    (t64908, t64926, t64998, t65005, t65015, t68280, t74445, t74475)
}
