//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 951/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk951<F: Float>(t1060: F, t22459: F, t4609: F, t695: F, t8500: F, t11259: F, t1417: F, t8916: F, t2487: F, t6714: F, t1648: F, t4604: F, t8514: F, t1824: F, t8510: F) -> (F, F, F, F, F, F, F) {
    let t22460 = t22459 * t1060;
    let t22461 = t4609 * t22460;
    let t22464 = t8500 * t695;
    let t22465 = t22464 * t1060;
    let t22466 = t11259 * t22465;
    let t22469 = t1417 * t8916;
    let t22473 = t4609 * t6714 * t2487;
    let t22477 = t4604 * t8514 * t1648;
    let t22481 = t4609 * t8514 * t1824;
    let t22484 = t8510 * t1060;
    (t22461, t22466, t22469, t22473, t22477, t22481, t22484)
}
