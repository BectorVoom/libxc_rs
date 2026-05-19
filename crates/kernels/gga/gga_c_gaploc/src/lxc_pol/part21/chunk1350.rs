//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1350/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1350<F: Float>(t15482: F, t20549: F, t35101: F, t10540: F, t18067: F, t2365: F, t25730: F, t4391: F, t25580: F, t20671: F, t27007: F, t31047: F) -> (F, F, F, F, F) {
    let t35104 = F::cast_from(0.34082600847929555269e0_f64) * t20549 * t15482 * t35101;
    let t35109 = t18067 * t10540;
    let t35110 = F::cast_from(0.59584149919750711116e-1_f64) * t35109;
    let t35112 = t4391 * t2365 * t25730;
    let t35113 = F::cast_from(0.59584149919750711116e-1_f64) * t35112;
    let t35115 = t4391 * t2365 * t25580;
    let t35116 = F::cast_from(0.29792074959875355558e-1_f64) * t35115;
    let t35119 = t31047 * t20671 * t27007;
    (t35104, t35110, t35113, t35116, t35119)
}
