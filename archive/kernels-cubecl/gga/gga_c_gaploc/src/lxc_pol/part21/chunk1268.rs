//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1268/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1268<F: Float>(t1890: F, t1966: F, t32435: F, t590: F, t326: F, t32889: F, t7394: F, t28412: F, t8970: F, t913: F, t1022: F, t15499: F) -> (F, F, F, F) {
    let t33526 = F::cast_from(0.51123901271894332902e1_f64) * t1966 * t1890 * t32435 * t590;
    let t33529 = F::cast_from(0.92023022289409799224e1_f64) * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = F::cast_from(0.59584149919750711116e-1_f64) * t33531;
    let t33533 = t15499 * t1022;
    (t33526, t33529, t33532, t33533)
}
