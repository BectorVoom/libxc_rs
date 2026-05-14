//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 888/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk888<F: Float>(t1916: F, t880: F, t2141: F, t1743: F, t2084: F, t2139: F, t27: F, t1528: F, t236: F, t3351: F, t618: F, t9210: F, t7720: F, t9932: F, t39277: F, t8836: F) -> (F, F, F, F, F) {
    let t47237 = t1916 * t880;
    let t47238 = t47237 * t2141;
    let t47242 = t2139 * t27 * t2084 * t1743;
    let t47263 = t3351 * t9210 * t236 * t618 * t1528;
    let t47265 = t7720 * t9932;
    let t47267 = t39277 * t8836;
    (t47238, t47242, t47263, t47265, t47267)
}
