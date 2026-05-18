//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1297/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1297<F: Float>(t3399: F, t4637: F, t10375: F, t1641: F, t1445: F, t25556: F, t574: F, t874: F, t2293: F, t7980: F, t2859: F, t31153: F) -> (F, F, F, F, F) {
    let t34067 = F::new(0.11502877786176224903e2) * t4637 * t3399;
    let t34070 = F::new(0.92023022289409799224e1) * t1641 * t10375;
    let t34074 = F::new(0.46011511144704899612e1) * t574 * t1445 * t25556 * t874;
    let t34078 = F::new(0.92023022289409799224e1) * t574 * t1445 * t7980 * t2293;
    let t34087 = F::new(0.10725146985555128001e1) * t2859 * t31153;
    (t34067, t34070, t34074, t34078, t34087)
}
