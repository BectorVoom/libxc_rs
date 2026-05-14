//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 926/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk926<F: Float>(t2019: F, t2020: F, t9750: F, t2010: F, t2012: F, t6627: F, t13283: F, t2061: F, t10082: F, t236: F, t321: F, t3351: F, t35155: F, t1907: F, t498: F, t7230: F, t7231: F) -> (F, F, F, F, F) {
    let t47439 = t2019 * t2020 * t9750;
    let t47442 = t2010 * t2012 * t6627;
    let t47445 = t13283 * t2061;
    let t47450 = t3351 * t35155 * t236 * t10082 * t321;
    let t47455 = t7230 * t7231 * t236 * t1907 * t498;
    (t47439, t47442, t47445, t47450, t47455)
}
