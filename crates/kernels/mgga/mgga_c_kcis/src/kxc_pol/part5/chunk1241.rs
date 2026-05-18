//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1241/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1241<F: Float>(t20157: F, t20160: F, t20162: F, t20165: F, t20167: F, t20170: F, t20174: F, t20176: F, t20179: F, t20181: F, t20183: F, t20186: F, t20188: F, t20192: F, t20195: F, t20198: F, t20201: F, t20203: F, t20206: F) -> F {
    let t20809 = -F::new(0.9375e-1) * t20157 + F::new(0.375e0) * t20160 + F::new(0.26979166666666666666e-1) * t20162 + F::new(0.25e0) * t20165 + F::new(0.1875e0) * t20167 + F::new(0.89930555555555555553e-2) * t20170 + F::new(0.101171875e-1) * t20174 - F::new(0.13489583333333333333e-1) * t20176 - F::new(0.625e-1) * t20179 + F::new(0.625e-1) * t20181 - F::new(0.1875e0) * t20183 + F::new(0.55555555555555555555e-1) * t20186 - F::new(0.13489583333333333333e-1) * t20188 - F::new(0.13489583333333333333e-1) * t20192 - F::new(0.53958333333333333333e-1) * t20195 + F::new(0.44965277777777777777e-2) * t20198 + F::new(0.13489583333333333333e-1) * t20201 + F::new(0.14388888888888888889e0) * t20203 + F::new(0.13489583333333333333e-1) * t20206;
    t20809
}
