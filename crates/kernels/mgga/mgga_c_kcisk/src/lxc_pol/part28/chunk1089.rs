//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1089/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1089<F: Float>(t24948: F, t5491: F, t7246: F, t18356: F, t2013: F, t24926: F, t24931: F, t24936: F, t24940: F, t24945: F, t5471: F, t7581: F, t7591: F, t7606: F, t7611: F, t7629: F, t9214: F) -> (F,) {
    let t24949 = t5491 * t24948;
    let t24950 = t7246 * t24949;
    let t24960 = 0.39979530480394038253e-2 * t24926 + 0.23987718288236422951e-1 * t7581 * t7606 + 0.71963154864709268852e-1 * t2013 * t24931 + 0.17990788716177317213e-1 * t2013 * t24936 + 0.53972366148531951639e-1 * t2013 * t24940 - 0.17990788716177317213e-1 * t2013 * t24945 + 0.35981577432354634426e-1 * t2013 * t24950 - 0.17990788716177317213e-1 * t5471 * t9214 + 0.47975436576472845903e-1 * t7591 * t7629 - 0.39979530480394038252e-2 * t18356 - 0.17990788716177317213e-1 * t7581 * t7611;
    (t24960,)
}
