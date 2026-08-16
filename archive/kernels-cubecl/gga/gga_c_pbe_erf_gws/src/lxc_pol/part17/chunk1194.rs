//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1194/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1194<F: Float>(t2250: F, t3969: F, t933: F, t2191: F, t3065: F, t2159: F, t14028: F, t2308: F, t14063: F, t6257: F, t14092: F, t6569: F) -> (F, F, F, F, F, F) {
    let t51306 = t2250 * t3969 * t933;
    let t51309 = t3065 * t2191;
    let t51312 = t3065 * t2159;
    let t51315 = t14028 * t2308;
    let t51317 = t14063 * t6257;
    let t51325 = t14092 * t6569;
    (t51306, t51309, t51312, t51315, t51317, t51325)
}
