//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1193/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1193<F: Float>(t10371: F, t1641: F, t10385: F, t1580: F, t1445: F, t31857: F, t597: F, t2778: F, t574: F, t6393: F, t3399: F, t4637: F, t10452: F, t10488: F, t1328: F, t1450: F, t31828: F, t34025: F, t34032: F, t34036: F, t34038: F, t34045: F, t34052: F, t34054: F, t447: F, t4527: F, t4614: F, t4771: F) -> (F,) {
    let t34056 = 0.92023022289409799224e1 * t1641 * t10371;
    let t34058 = 0.23005755572352449806e2 * t1580 * t10385;
    let t34061 = 0.11502877786176224903e2 * t597 * t1445 * t31857;
    let t34065 = 0.46011511144704899612e1 * t574 * t1445 * t2778 * t6393;
    let t34067 = 0.11502877786176224903e2 * t4637 * t3399;
    let t34068 = -t34025 - 0.12269736305254639896e2 * t574 * t4614 * t10488 - t34032 - t34036 - t34038 - 0.46011511144704899612e1 * t4771 * t10452 - 0.46011511144704899612e1 * t1450 * t1445 * t31828 * t447 + 0.27606906686822939767e2 * t4527 * t1445 * t34045 * t1328 - t34052 - t34054 - t34056 + t34058 + t34061 - t34065 + t34067;
    (t34068,)
}
