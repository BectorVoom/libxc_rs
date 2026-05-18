//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 689/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk689<F: Float>(t11167: F, t874: F, t1445: F, t574: F, t11434: F, t901: F, t11430: F, t13261: F, t597: F, t2366: F, t3529: F, t2365: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13371 = t11167 * t874;
    let t13372 = t1445 * t13371;
    let t13374 = F::new(0.46011511144704899612e1) * t574 * t13372;
    let t13378 = t11434 * t901;
    let t13379 = F::new(0.14896037479937677779e-1) * t13378;
    let t13380 = t11430 * t901;
    let t13381 = F::new(0.14896037479937677779e-1) * t13380;
    let t13383 = t1445 * t13261;
    let t13385 = F::new(0.11502877786176224903e2) * t597 * t13383;
    let t13386 = t2366 * t3529;
    let t13387 = t2365 * t13386;
    (t13371, t13372, t13374, t13379, t13381, t13383, t13385, t13386, t13387)
}
