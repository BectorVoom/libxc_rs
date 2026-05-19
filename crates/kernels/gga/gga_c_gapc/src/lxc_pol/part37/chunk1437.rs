//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1437/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1437<F: Float>(t34176: F, t34178: F, t34193: F, t34200: F, t36849: F, t36850: F, t36851: F, t36854: F, t36855: F, t36856: F, t36857: F, t34227: F, t34230: F, t36862: F, t36863: F, t36864: F, t36865: F, t36866: F, t36867: F, t36868: F, t36869: F, t36870: F) -> (F, F) {
    let t38805 = t36849 + t36850 + t36851 + F::cast_from(0.36231816839129402172e-6_f64) * t34176 + F::cast_from(0.72463633678258804344e-6_f64) * t34178 - t36854 + t36855 + t36856 - t36857 + F::cast_from(0.7379489474826388889e-6_f64) * t34193 - F::cast_from(0.38527756621470067413e-7_f64) * t34200;
    let t38809 = t36862 + t36863 + t36864 + t36865 + t36866 - t36867 - t36868 - t36869 - t36870 + F::cast_from(0.84337022569444444446e-6_f64) * t34227 - F::cast_from(0.7379489474826388889e-6_f64) * t34230;
    (t38805, t38809)
}
