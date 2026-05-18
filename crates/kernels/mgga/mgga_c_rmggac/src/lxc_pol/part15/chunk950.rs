//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 950/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk950<F: Float>(t1734: F, t2084: F, t2134: F, t27: F, t2286: F, t38355: F, t7720: F, t9935: F, t10106: F, t16043: F, t10088: F, t2144: F, t3351: F, t352: F, t7231: F) -> (F, F, F, F, F) {
    let t45775 = t2134 * t27 * t2084 * t1734;
    let t45777 = t38355 * t2286;
    let t45779 = t7720 * t9935;
    let t45781 = t16043 * t10106;
    let t45788 = t3351 * t7231 * t2144 * t10088 * t352;
    (t45775, t45777, t45779, t45781, t45788)
}
