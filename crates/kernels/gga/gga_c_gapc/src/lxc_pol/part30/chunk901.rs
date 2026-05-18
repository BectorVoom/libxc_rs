//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 901/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk901<F: Float>(t9731: F, t9734: F, t9742: F, t9745: F, t9748: F, t9751: F, t9754: F, t9758: F, t9761: F, t9764: F, t9766: F, t9768: F, t9771: F) -> F {
    let t10932 = F::new(0.34752370105806885418e-3) * t9731 - F::new(0.38647271295071362317e-7) * t9734 + F::new(0.43047021936487268522e-6) * t9742 + F::new(0.17376185052903442709e-3) * t9745 - F::new(0.13900948042322754167e-3) * t9748 - F::new(0.13900948042322754167e-3) * t9751 + F::new(0.41702844126968262501e-3) * t9754 + F::new(0.10005428175813516294e-8) * t9758 + F::new(0.15458908518028544927e-5) * t9761 - F::new(0.51491428373437201896e-5) * t9764 - F::new(0.34752370105806885418e-3) * t9766 + F::new(0.28960308421505737848e-5) * t9768 - F::new(0.45018799441230669486e-7) * t9771;
    t10932
}
