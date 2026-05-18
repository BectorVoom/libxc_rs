//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1179/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1179<F: Float>(t2147: F, t26307: F, t3332: F, t38096: F, t38099: F, t38111: F, t38114: F, t40145: F, t40149: F, t40151: F, t40153: F, t40156: F, t40158: F, t40162: F) -> F {
    let t40165 = t2147 * t3332 * t26307;
    let t40167 = F::new(0.21831846657716620896e-2) * t40145 - F::new(0.46574606203128791246e-1) * t38096 - F::new(0.13972381860938637374e0) * t38099 - F::new(0.87327386630866483584e-2) * t40149 - F::new(0.13099107994629972538e-1) * t40151 + F::new(0.43663693315433241792e-2) * t40153 - t40156 + t40158 - F::new(0.12805040077930161442e0) * t38111 - F::new(0.23115257973478049502e0) * t38114 - F::new(0.13972381860938637373e0) * t40162 + F::new(0.21831846657716620896e-2) * t40165;
    t40167
}
