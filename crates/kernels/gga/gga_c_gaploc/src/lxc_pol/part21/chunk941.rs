//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 941/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk941<F: Float>(t8097: F, t874: F, t1445: F, t1562: F, t1641: F, t3391: F, t7980: F, t574: F, t2293: F, t2778: F, t1580: F, t3399: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10364 = t8097 * t874;
    let t10365 = t1445 * t10364;
    let t10367 = F::new(0.69017266717057349418e1) * t1562 * t10365;
    let t10369 = F::new(0.46011511144704899612e1) * t1641 * t3391;
    let t10370 = t7980 * t874;
    let t10371 = t1445 * t10370;
    let t10373 = F::new(0.46011511144704899612e1) * t574 * t10371;
    let t10374 = t2778 * t2293;
    let t10375 = t1445 * t10374;
    let t10377 = F::new(0.46011511144704899612e1) * t574 * t10375;
    let t10381 = F::new(0.11502877786176224903e2) * t1580 * t3399;
    (t10364, t10365, t10367, t10369, t10370, t10371, t10373, t10374, t10375, t10377, t10381)
}
