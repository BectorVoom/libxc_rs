//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1293/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1293<F: Float>(t13781: F, t15144: F, t3038: F, t3972: F, t14696: F, t39061: F, t3975: F, t38036: F, t6472: F, t820: F, t14767: F, t3047: F) -> (F, F, F, F) {
    let t56651 = t3972 * t13781 * t3038 * t15144;
    let t56657 = t3972 * t3975 * t39061 * t14696;
    let t56667 = t3972 * t3975 * t38036 * t6472 * t820;
    let t56674 = t14767 * t3047;
    (t56651, t56657, t56667, t56674)
}
