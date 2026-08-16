//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1270/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1270<F: Float>(t3928: F, t944: F, t3717: F, t1172: F, t810: F, t14767: F, t2503: F, t29260: F, t3808: F, t3972: F, t3975: F, t45096: F, t51555: F) -> (F, F, F, F, F, F) {
    let t56042 = t3928 * t944;
    let t56046 = t3717 * t944;
    let t56053 = t1172 * t810;
    let t56061 = t14767 * t2503;
    let t56067 = t3972 * t3975 * t3808 * t29260;
    let t56070 = t51555 * t3975 * t45096;
    (t56042, t56046, t56053, t56061, t56067, t56070)
}
