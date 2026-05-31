//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 953/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk953<F: Float>(t7386: F, t7389: F, t5525: F, t5560: F, t5563: F, t5566: F, t5783: F, t5790: F, t7357: F, t7393: F, t7397: F, t7401: F) -> F {
    let t7434 = F::cast_from(0.32862666666666666666e0_f64) * t7386;
    let t7435 = F::cast_from(0.32862666666666666666e0_f64) * t7389;
    let t7442 = -F::cast_from(0.29896666666666666667e0_f64) * t5525 + F::cast_from(0.39862222222222222223e0_f64) * t7357 - t7434 - t7435 + F::cast_from(0.24647e0_f64) * t7393 + F::cast_from(0.49294e0_f64) * t7397 + F::cast_from(0.24647e0_f64) * t7401 - t5783 - t5790 + F::cast_from(0.54771111111111111111e0_f64) * t5560 - F::cast_from(0.16431333333333333333e0_f64) * t5563 - F::cast_from(0.16431333333333333333e0_f64) * t5566;
    t7442
}
