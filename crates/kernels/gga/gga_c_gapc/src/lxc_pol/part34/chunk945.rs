//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 945/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk945<F: Float>(t9731: F, t9734: F, t9742: F, t9745: F, t9748: F, t9751: F, t9754: F, t9758: F, t9761: F, t9764: F, t9766: F, t9768: F, t9771: F) -> F {
    let t10932 = F::cast_from(0.34752370105806885418e-3_f64) * t9731 - F::cast_from(0.38647271295071362317e-7_f64) * t9734 + F::cast_from(0.43047021936487268522e-6_f64) * t9742 + F::cast_from(0.17376185052903442709e-3_f64) * t9745 - F::cast_from(0.13900948042322754167e-3_f64) * t9748 - F::cast_from(0.13900948042322754167e-3_f64) * t9751 + F::cast_from(0.41702844126968262501e-3_f64) * t9754 + F::cast_from(0.10005428175813516294e-8_f64) * t9758 + F::cast_from(0.15458908518028544927e-5_f64) * t9761 - F::cast_from(0.51491428373437201896e-5_f64) * t9764 - F::cast_from(0.34752370105806885418e-3_f64) * t9766 + F::cast_from(0.28960308421505737848e-5_f64) * t9768 - F::cast_from(0.45018799441230669486e-7_f64) * t9771;
    t10932
}
