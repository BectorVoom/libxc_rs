//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 918/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk918<F: Float>(t29007: F, t29049: F, t29301: F, t29320: F, t1791: F, t16676: F, t8486: F, t11213: F, t29274: F, t1800: F, t1869: F, t28663: F, t6666: F) -> (F, F, F, F) {
    let t29322 = t29007 + t29049 + t29301 + t29320;
    let t29323 = t29322 * t1791;
    let t29326 = t16676 * t8486;
    let t29328 = t11213 * t29274;
    let t29329 = t1800 * t29328;
    let t29330 = t1869 * t29329;
    let t29332 = t6666 * t28663;
    (t29323, t29326, t29330, t29332)
}
