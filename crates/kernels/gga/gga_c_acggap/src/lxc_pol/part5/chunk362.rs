//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 362/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk362<F: Float>(t1240: F, t322: F, t381: F, t452: F, t879: F, t180: F, t939: F, t945: F, t394: F, t441: F) -> (F, F, F, F, F, F, F) {
    let t1241 = t1240 * t322;
    let t1242 = t381 * t1241;
    let t1244 = t452 * t879;
    let t1246 = F::new(0.65854491829355115987e0) * t381 * t1244;
    let t1247 = t939 * t180;
    let t1248 = t1247 * t945;
    let t1251 = t394 * t441;
    (t1241, t1242, t1244, t1246, t1247, t1248, t1251)
}
