//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 694/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk694<F: Float>(t13359: F, t1445: F, t1645: F, t2492: F, t11359: F, t13276: F, t1562: F, t11167: F, t874: F, t574: F, t3358: F, t11434: F, t901: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13360 = t1445 * t13359;
    let t13363 = t1645 * t2492;
    let t13365 = F::new(0.42900587942220512003e1) * t11359 * t13363;
    let t13368 = t1445 * t13276;
    let t13370 = F::new(0.62115540045351614476e2) * t1562 * t13368;
    let t13371 = t11167 * t874;
    let t13372 = t1445 * t13371;
    let t13374 = F::new(0.46011511144704899612e1) * t574 * t13372;
    let t13375 = t1645 * t3358;
    let t13378 = t11434 * t901;
    (t13360, t13363, t13365, t13368, t13370, t13371, t13372, t13374, t13375, t13378)
}
