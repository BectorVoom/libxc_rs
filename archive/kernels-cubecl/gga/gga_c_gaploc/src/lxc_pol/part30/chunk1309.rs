//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1309/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1309<F: Float>(t326: F, t32889: F, t7394: F, t28412: F, t8970: F, t913: F, t1022: F, t15499: F, t28640: F, t7419: F, t3005: F, t7383: F, t9800: F) -> (F, F, F, F) {
    let t33529 = F::cast_from(0.92023022289409799224e1_f64) * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = F::cast_from(0.59584149919750711116e-1_f64) * t33531;
    let t33533 = t15499 * t1022;
    let t33535 = t28640 * t33533 * t7419;
    let t33536 = F::cast_from(0.23005755572352449806e1_f64) * t33535;
    let t33538 = t9800 * t3005 * t7383;
    (t33529, t33532, t33536, t33538)
}
