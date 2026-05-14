//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1262/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1262<F: Float>(t35228: F, t10434: F, t1391: F, t2487: F, t1441: F, t31412: F, t31414: F, t31417: F, t34567: F, t35192: F, t35199: F, t35201: F, t35206: F, t35209: F, t35211: F, t35214: F, t35219: F, t35220: F, t35226: F, t590: F, t6710: F, t6711: F) -> (F,) {
    let t35229 = 0.51123901271894332902e0 * t35228;
    let t35231 = t2487 * t1391 * t10434;
    let t35232 = 0.2698205900461089792e0 * t35231;
    let t35233 = -t35192 - 0.23005755572352449806e2 * t6710 * t6711 * t34567 - t35199 - t35201 - t35206 - t35209 + t35211 - t35214 - t35219 + 0.2044956050875773316e1 * t1441 * t35220 * t590 - t31412 - t31414 - t31417 - t35226 - t35229 + t35232;
    (t35233,)
}
