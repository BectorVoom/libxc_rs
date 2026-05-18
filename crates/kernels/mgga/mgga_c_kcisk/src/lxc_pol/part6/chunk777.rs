//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 777/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk777<F: Float>(t17276: F, t1773: F, t2448: F, t3934: F, t654: F, t10879: F, t2459: F, t10935: F, t1224: F, t2364: F) -> (F, F, F, F) {
    let t17277 = t1773 * t17276;
    let t17317 = t2448 * t654 * t3934;
    let t17326 = t10879 * t2459;
    let t17327 = t1773 * t17326;
    let t17382 = t1224 * t10935 * t2364;
    (t17277, t17317, t17327, t17382)
}
