//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 858/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk858<F: Float>(t1457: F, t1572: F, t41778: F, t12922: F, t26939: F, t12926: F, t1641: F, t1445: F, t3116: F, t574: F, t7980: F, t2778: F, t9127: F) -> (F, F, F, F, F) {
    let t42278 = F::new(0.71500979903700853338e0) * t1572 * t1457 * t41778;
    let t42282 = F::new(0.42900587942220512003e1) * t26939 * t12922;
    let t42284 = F::new(0.46011511144704899612e1) * t1641 * t12926;
    let t42288 = F::new(0.46011511144704899612e1) * t574 * t1445 * t7980 * t3116;
    let t42292 = F::new(0.46011511144704899612e1) * t574 * t1445 * t2778 * t9127;
    (t42278, t42282, t42284, t42288, t42292)
}
