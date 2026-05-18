//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1270/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1270<F: Float>(t1890: F, t1966: F, t32435: F, t590: F, t326: F, t32889: F, t7394: F, t28412: F, t8970: F, t913: F, t1022: F, t15499: F) -> (F, F, F, F) {
    let t33526 = F::new(0.51123901271894332902e1) * t1966 * t1890 * t32435 * t590;
    let t33529 = F::new(0.92023022289409799224e1) * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = F::new(0.59584149919750711116e-1) * t33531;
    let t33533 = t15499 * t1022;
    (t33526, t33529, t33532, t33533)
}
