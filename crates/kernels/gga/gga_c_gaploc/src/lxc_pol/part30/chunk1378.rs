//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1378/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1378<F: Float>(t1359: F, t3338: F, t34411: F, t6716: F, t6717: F, t10409: F, t31356: F, t2482: F, t2792: F, t9263: F, t6895: F, t993: F) -> (F, F, F, F, F) {
    let t34478 = t1359 * t3338;
    let t34484 = F::new(0.69017266717057349418e1) * t6716 * t6717 * t34411;
    let t34485 = t31356 * t10409;
    let t34486 = F::new(0.76685851907841499352e0) * t34485;
    let t34488 = t9263 * t2792 * t2482;
    let t34489 = F::new(0.76685851907841499352e0) * t34488;
    let t34491 = t9263 * t993 * t6895;
    (t34478, t34484, t34486, t34489, t34491)
}
