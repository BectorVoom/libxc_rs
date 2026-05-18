//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1350/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1350<F: Float>(t3399: F, t4637: F, t10452: F, t10488: F, t1328: F, t1445: F, t1450: F, t31828: F, t34025: F, t34032: F, t34036: F, t34038: F, t34045: F, t34052: F, t34054: F, t34056: F, t34058: F, t34061: F, t34065: F, t447: F, t4527: F, t4614: F, t4771: F, t574: F) -> F {
    let t34067 = F::new(0.11502877786176224903e2) * t4637 * t3399;
    let t34068 = -t34025 - F::new(0.12269736305254639896e2) * t574 * t4614 * t10488 - t34032 - t34036 - t34038 - F::new(0.46011511144704899612e1) * t4771 * t10452 - F::new(0.46011511144704899612e1) * t1450 * t1445 * t31828 * t447 + F::new(0.27606906686822939767e2) * t4527 * t1445 * t34045 * t1328 - t34052 - t34054 - t34056 + t34058 + t34061 - t34065 + t34067;
    t34068
}
