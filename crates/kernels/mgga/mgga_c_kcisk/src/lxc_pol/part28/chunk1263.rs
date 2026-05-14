//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1263/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1263<F: Float>(t5217: F, t7291: F, t11196: F, t2399: F, t4822: F, t6884: F, t5531: F, t7654: F, t11700: F, t2541: F, t12351: F, t2656: F, t8964: F, t1907: F, t24079: F, t2041: F, t25151: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63008 = t7291 * t5217;
    let t64905 = t2399 * t11196;
    let t64908 = t6884 * t4822;
    let t64998 = t7654 * t5531;
    let t65005 = t2541 * t11700;
    let t65015 = t2656 * t12351;
    let t65157 = t8964 * t5217;
    let t65168 = t24079 * t1907;
    let t65181 = t25151 * t2041;
    (t63008, t64905, t64908, t64998, t65005, t65015, t65157, t65168, t65181)
}
