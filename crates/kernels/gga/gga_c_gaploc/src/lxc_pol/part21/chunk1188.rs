//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1188/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1188<F: Float>(t35141: F, t26822: F, t901: F, t10315: F, t20445: F, t20168: F, t31540: F, t20158: F, t31735: F, t20172: F, t2854: F, t590: F, t6519: F, t2875: F, t544: F, t6514: F) -> (F, F, F, F, F, F, F) {
    let t35142 = 0.29792074959875355558e-1 * t35141;
    let t35143 = t26822 * t901;
    let t35144 = 0.14896037479937677779e-1 * t35143;
    let t35146 = 0.14300195980740170668e1 * t20445 * t10315;
    let t35172 = 0.51123901271894332902e1 * t20168 * t31540;
    let t35174 = 0.2044956050875773316e1 * t20158 * t31735;
    let t35178 = 0.30674340763136599742e1 * t20172 * t2854 * t6519 * t590;
    let t35180 = t544 * t6514 * t2875;
    (t35142, t35144, t35146, t35172, t35174, t35178, t35180)
}
