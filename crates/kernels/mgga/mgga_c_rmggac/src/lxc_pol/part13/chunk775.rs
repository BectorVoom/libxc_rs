//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 775/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk775<F: Float>(t35979: F, t3814: F, t2064: F, t5245: F, t848: F, t797: F, t34805: F, t648: F, t35765: F, t793: F, t305: F, t35590: F) -> (F, F, F, F, F, F, F) {
    let t35980 = t3814 * t35979;
    let t35989 = t5245 * t2064;
    let t36012 = t2064 * t848;
    let t36013 = t797 * t36012;
    let t36034 = t648 * t34805;
    let t36045 = t793 * t35765;
    let t36058 = t305 * t35590;
    (t35980, t35989, t36012, t36013, t36034, t36045, t36058)
}
