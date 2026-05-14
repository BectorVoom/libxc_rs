//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 566/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk566<F: Float>(t10597: F, t9267: F, t2299: F, t2875: F, t544: F, t1424: F, t3358: F, t590: F, t10564: F, t10570: F, t10573: F, t10578: F, t10584: F, t10587: F, t10591: F, t10594: F, t1441: F, t1580: F, t1599: F, t1641: F, t193: F, t3372: F, t3387: F, t3403: F, t3415: F, t541: F, t557: F, t574: F, t597: F) -> (F, F) {
    let t10598 = t9267 * t10597;
    let t10599 = 0.9585731488480187419e0 * t10598;
    let t10600 = t2299 * t2875;
    let t10601 = t544 * t10600;
    let t10603 = 0.39722766613167140743e-1 * t10601 * t1424;
    let t10604 = t3358 * t590;
    let t10607 = 0.23833659967900284446e0 * t3372 * t541 - 0.30674340763136599741e1 * t574 * t10564 + 0.23005755572352449806e1 * t1580 * t3415 + 0.23005755572352449806e1 * t597 * t10570 + 0.30674340763136599741e1 * t597 * t10573 - 0.35750489951850426669e0 * t1599 * t3387 - 0.35750489951850426669e0 * t557 * t10578 - 0.23005755572352449806e1 * t1641 * t3403 - 0.23005755572352449806e1 * t574 * t10584 + 0.35750489951850426669e0 * t10587 * t193 + 0.35750489951850426669e0 * t10591 * t193 - 0.23833659967900284446e0 * t557 * t10594 + t10599 - t10603 + 0.51123901271894332902e0 * t1441 * t10604;
    (t10600, t10607)
}
