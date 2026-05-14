//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 825/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk825<F: Float>(t42366: F, t34604: F, t544: F, t9287: F, t10532: F, t10533: F, t41726: F, t34400: F, t34401: F, t12938: F, t2464: F, t587: F, t40514: F, t40517: F, t9065: F, t986: F) -> (F, F, F, F, F, F, F, F) {
    let t42367 = 0.29792074959875355558e-1 * t42366;
    let t42369 = t544 * t34604 * t9287;
    let t42370 = 0.29792074959875355558e-1 * t42369;
    let t42373 = 0.38649669361552115674e3 * t10532 * t10533 * t41726;
    let t42376 = 0.13803453343411469884e3 * t34400 * t34401 * t41726;
    let t42378 = t587 * t2464 * t12938;
    let t42379 = 0.63904876589867916128e-1 * t42378;
    let t42380 = 0.59584149919750711116e-1 * t40514;
    let t42381 = 0.25561950635947166451e0 * t40517;
    let t42382 = t9065 * t986;
    (t42367, t42370, t42373, t42376, t42379, t42380, t42381, t42382)
}
