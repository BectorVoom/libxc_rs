//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1128/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1128<F: Float>(t2019: F, t2029: F, t8807: F, t31142: F, t8810: F, t1314: F, t361: F, t8806: F, t142: F, t4578: F, t4483: F, t1318: F, t7436: F) -> (F, F, F, F, F, F) {
    let t36039 = t2019 * t2029 * t8807;
    let t36041 = t31142 * t8810;
    let t36044 = t8806 * t361 * t1314;
    let t36047 = t8806 * t142 * t4578;
    let t36050 = t8806 * t142 * t4483;
    let t36053 = t7436 * t361 * t1318;
    (t36039, t36041, t36044, t36047, t36050, t36053)
}
