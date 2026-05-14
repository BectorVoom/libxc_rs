//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1134/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1134<F: Float>(t20025: F, t32088: F, t3937: F, t3491: F, t388: F, t1308: F) -> (F, F, F, F) {
    let t32089 = t32088 * t20025;
    let t32090 = t3937 * t32089;
    let t32095 = t3491 * t388;
    let t32096 = t32095 * t1308;
    (t32089, t32090, t32095, t32096)
}
