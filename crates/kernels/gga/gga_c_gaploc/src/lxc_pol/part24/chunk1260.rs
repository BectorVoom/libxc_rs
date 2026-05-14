//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1260/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1260<F: Float>(t20168: F, t31540: F, t20158: F, t31735: F, t20172: F, t2854: F, t590: F, t6519: F, t2875: F, t544: F, t6514: F, t1367: F, t20901: F, t10547: F, t6820: F, t204: F, t2476: F, t34411: F) -> (F, F, F, F, F, F) {
    let t35172 = 0.51123901271894332902e1 * t20168 * t31540;
    let t35174 = 0.2044956050875773316e1 * t20158 * t31735;
    let t35178 = 0.30674340763136599742e1 * t20172 * t2854 * t6519 * t590;
    let t35180 = t544 * t6514 * t2875;
    let t35183 = 0.55611873258433997041e0 * t35180 * t20901 * t1367;
    let t35185 = 0.25025342966295298669e1 * t10547 * t6820;
    let t35188 = 0.46011511144704899612e1 * t2476 * t204 * t34411;
    (t35172, t35174, t35178, t35183, t35185, t35188)
}
