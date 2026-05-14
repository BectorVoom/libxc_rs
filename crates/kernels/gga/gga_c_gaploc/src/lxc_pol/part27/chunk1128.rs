//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1128/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1128<F: Float>(t326: F, t32889: F, t7394: F, t28412: F, t8970: F, t913: F, t1022: F, t15499: F, t28640: F, t7419: F, t3005: F, t7383: F, t9800: F, t3484: F, t6021: F, t10973: F, t2194: F) -> (F, F, F, F, F, F) {
    let t33529 = 0.92023022289409799224e1 * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    let t33532 = 0.59584149919750711116e-1 * t33531;
    let t33533 = t15499 * t1022;
    let t33535 = t28640 * t33533 * t7419;
    let t33536 = 0.23005755572352449806e1 * t33535;
    let t33538 = t9800 * t3005 * t7383;
    let t33539 = 0.9585731488480187419e0 * t33538;
    let t33544 = 0.46011511144704899612e1 * t6021 * t3484;
    let t33546 = 0.92023022289409799224e1 * t2194 * t10973;
    (t33529, t33532, t33536, t33539, t33544, t33546)
}
