//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1078/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1078<F: Float>(t11537: F, t20372: F, t5059: F, t1: F, t1457: F, t169: F, t1736: F, t11344: F, t11597: F, t3008: F, t3060: F, t1030: F, t11591: F, t144: F, t1461: F, t8709: F) -> (F, F, F, F, F) {
    let t34921 = t11537 * t20372 * t5059;
    let t34925 = t169 * t1457 * t1736 * t1;
    let t34926 = t34925 * t11344;
    let t34929 = t3060 * t11597 * t3008;
    let t34934 = t1030 * t1461 * t8709 * t144 * t11591;
    (t34921, t34925, t34926, t34929, t34934)
}
