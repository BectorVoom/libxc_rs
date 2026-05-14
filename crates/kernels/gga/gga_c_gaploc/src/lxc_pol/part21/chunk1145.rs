//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1145/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1145<F: Float>(t1445: F, t31857: F, t597: F, t2778: F, t574: F, t6393: F, t3399: F, t4637: F, t10375: F, t1641: F, t25556: F, t874: F, t2293: F, t7980: F, t2859: F, t31153: F) -> (F, F, F, F, F, F, F) {
    let t34061 = 0.11502877786176224903e2 * t597 * t1445 * t31857;
    let t34065 = 0.46011511144704899612e1 * t574 * t1445 * t2778 * t6393;
    let t34067 = 0.11502877786176224903e2 * t4637 * t3399;
    let t34070 = 0.92023022289409799224e1 * t1641 * t10375;
    let t34074 = 0.46011511144704899612e1 * t574 * t1445 * t25556 * t874;
    let t34078 = 0.92023022289409799224e1 * t574 * t1445 * t7980 * t2293;
    let t34087 = 0.10725146985555128001e1 * t2859 * t31153;
    (t34061, t34065, t34067, t34070, t34074, t34078, t34087)
}
