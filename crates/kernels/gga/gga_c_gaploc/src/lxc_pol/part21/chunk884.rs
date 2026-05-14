//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 884/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk884<F: Float>(t2389: F, t2898: F, t10314: F, t204: F, t2476: F, t594: F, t986: F, t1: F, t544: F, t2392: F, t2482: F, t2890: F, t9267: F, t2299: F, t2875: F, t1424: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10550 = t2898 * t2389;
    let t10551 = 0.29792074959875355558e-1 * t10550;
    let t10552 = t204 * t10314;
    let t10554 = 0.46011511144704899612e1 * t2476 * t10552;
    let t10555 = t594 * t986;
    let t10556 = t10555 * t1;
    let t10557 = t544 * t10556;
    let t10559 = 0.42900587942220512003e1 * t10557 * t2392;
    let t10597 = t2890 * t2482;
    let t10598 = t9267 * t10597;
    let t10599 = 0.9585731488480187419e0 * t10598;
    let t10600 = t2299 * t2875;
    let t10601 = t544 * t10600;
    let t10603 = 0.39722766613167140743e-1 * t10601 * t1424;
    (t10551, t10552, t10554, t10555, t10557, t10559, t10597, t10599, t10600, t10601, t10603)
}
