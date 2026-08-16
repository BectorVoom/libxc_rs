//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1292/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1292<F: Float>(t3391: F, t4634: F, t10371: F, t1641: F, t10385: F, t1580: F, t1445: F, t31857: F, t597: F, t2778: F, t574: F, t6393: F) -> (F, F, F, F, F) {
    let t34054 = F::cast_from(0.46011511144704899612e1_f64) * t4634 * t3391;
    let t34056 = F::cast_from(0.92023022289409799224e1_f64) * t1641 * t10371;
    let t34058 = F::cast_from(0.23005755572352449806e2_f64) * t1580 * t10385;
    let t34061 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t31857;
    let t34065 = F::cast_from(0.46011511144704899612e1_f64) * t574 * t1445 * t2778 * t6393;
    (t34054, t34056, t34058, t34061, t34065)
}
