//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1111/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1111<F: Float>(t36094: F, t36096: F, t46150: F, t46152: F, t46154: F, t46156: F, t46158: F, t46160: F, t46162: F, t46165: F, t46168: F, t46170: F, t46172: F, t46178: F, t46182: F, t46186: F) -> F {
    let t49082 = -F::cast_from(0.63504270469206447405e-2_f64) * t46150 + F::cast_from(0.1814407727691612783e-2_f64) * t46152 - F::cast_from(0.25401708187682578962e-2_f64) * t46154 - F::cast_from(0.25401708187682578962e-2_f64) * t46156 + F::cast_from(0.10160683275073031585e-1_f64) * t46158 - F::cast_from(0.15241024912609547377e-1_f64) * t46160 + F::cast_from(0.33868944250243438616e-2_f64) * t46162 + F::cast_from(0.9072038638458063915e-3_f64) * t46165 - F::cast_from(0.12700854093841289481e-2_f64) * t46168 - F::cast_from(0.12700854093841289481e-2_f64) * t46170 + F::cast_from(0.16934472125121719308e-2_f64) * t46172 + F::cast_from(0.66671395154821946451e-1_f64) * t36094 - F::cast_from(0.88895193539762595268e-1_f64) * t36096 + F::cast_from(0.24244143692662525982e-1_f64) * t46178 - F::cast_from(0.2419210303588817044e-2_f64) * t46182 - F::cast_from(0.45158592333657918155e-2_f64) * t46186;
    t49082
}
