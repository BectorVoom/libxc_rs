//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1039/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1039<F: Float>(t12213: F, t2417: F, t3306: F, t6781: F, t331: F, t8703: F, t2306: F, t3074: F, t3075: F, t837: F, t3039: F, t4384: F, t6792: F, t2395: F, t2494: F, t1105: F, t4417: F) -> (F, F, F, F, F, F, F, F) {
    let t22172 = t12213 * t2417;
    let t22192 = t6781 * t3306;
    let t22237 = t8703 * t331;
    let t22263 = t3074 * t2306 * t22237;
    let t22334 = t3075 * t837;
    let t22336 = t3074 * t2306 * t22334;
    let t22343 = t3039 * t4384;
    let t22379 = t3039 * t6792;
    let t22393 = t2395 * t2494;
    let t22410 = t4417 * t1105;
    (t22172, t22192, t22263, t22336, t22343, t22379, t22393, t22410)
}
