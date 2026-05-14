//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 602/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk602<F: Float>(t1801: F, t8939: F, t1873: F, t1869: F, t2441: F, t2527: F) -> (F, F, F, F) {
    let t8940 = t1801 * t8939;
    let t8941 = t1873 * t8940;
    let t8942 = t1869 * t8941;
    let t8946 = t2527 * t2441;
    (t8940, t8941, t8942, t8946)
}
