//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 751/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk751<F: Float>(t72177: F, t70612: F, t14953: F, t942: F, t3285: F, t934: F, t68751: F, t68808: F, t69027: F, t69082: F, t69085: F, t69674: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t72178 = F::cast_from(0.30487649791575028314e-3_f64) * t72177;
    let t72193 = F::cast_from(0.30487649791575028312e-3_f64) * t70612;
    let t73234 = t942 * t14953;
    let t73255 = t934 * t3285;
    let t73266 = F::cast_from(0.81700459932833791249e-6_f64) * t68751;
    let t73276 = F::cast_from(0.26021382394247697186e-4_f64) * t68808;
    let t73309 = F::cast_from(0.22800128353348965e-6_f64) * t69027;
    let t73321 = F::cast_from(0.69390353051327192495e-4_f64) * t69082;
    let t73322 = F::cast_from(0.65053455985619242964e-5_f64) * t69085;
    let t73344 = F::cast_from(0.1135168950387742861e-7_f64) * t69674;
    (t72178, t72193, t73234, t73255, t73266, t73276, t73309, t73321, t73322, t73344)
}
