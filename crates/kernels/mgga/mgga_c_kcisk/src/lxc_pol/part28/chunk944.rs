//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 944/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk944<F: Float>(t2063: F, t7069: F, t5185: F, t5184: F, t5182: F, t695: F, t8522: F, t1060: F, t4604: F, t11279: F, t1648: F, t8510: F, t11285: F, t1824: F, t10487: F, t7715: F) -> (F, F, F, F, F, F) {
    let t22368 = t2063 * t7069;
    let t22369 = t5185 * t22368;
    let t22370 = t5184 * t22369;
    let t22371 = t5182 * t22370;
    let t22373 = t8522 * t695;
    let t22374 = t22373 * t1060;
    let t22375 = t4604 * t22374;
    let t22379 = t11279 * t8510 * t1648;
    let t22383 = t11285 * t8510 * t1824;
    let t22386 = t10487 * t7715;
    (t22369, t22371, t22375, t22379, t22383, t22386)
}
