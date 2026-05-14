//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1036/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1036<F: Float>(t31054: F, t9274: F, t1265: F, t2487: F, t9438: F, t9448: F, t18970: F, t3163: F, t10530: F, t584: F, t6574: F, t123: F, t18313: F, t20369: F, t883: F, t6907: F, t888: F, t9263: F) -> (F, F, F, F, F, F) {
    let t31056 = 0.23005755572352449806e1 * t31054 * t9274;
    let t31065 = t2487 * t9438 * t9448 * t1265;
    let t31068 = 0.29792074959875355558e-1 * t18970 * t3163;
    let t31119 = t584 * t10530 * t6574;
    let t31120 = t18313 * t123;
    let t31124 = 0.46011511144704899612e1 * t31119 * t31120 * t883 * t20369;
    let t31126 = t9263 * t888 * t6907;
    (t31056, t31065, t31068, t31119, t31124, t31126)
}
