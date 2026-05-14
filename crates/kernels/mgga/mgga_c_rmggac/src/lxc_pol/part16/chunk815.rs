//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 815/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk815<F: Float>(t1763: F, t1970: F, t1971: F, t209: F, t476: F, t875: F, t10030: F, t7244: F, t10024: F, t498: F, t515: F, t7230: F, t7231: F, t9843: F, t321: F, t3352: F) -> (F, F, F, F, F) {
    let t45240 = t1970 * t1971 * t875 * t1763 * t476 * t209;
    let t45242 = t7244 * t10030;
    let t45244 = t7244 * t10024;
    let t45249 = t7230 * t7231 * t515 * t9843 * t498;
    let t45254 = t7230 * t3352 * t515 * t9843 * t321;
    (t45240, t45242, t45244, t45249, t45254)
}
