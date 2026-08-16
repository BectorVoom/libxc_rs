//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 865/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk865<F: Float>(t22574: F, t31300: F, t191: F, t192: F, t7166: F, t2020: F, t6997: F, t8607: F, t8562: F, t865: F, t2718: F, t225: F, t258: F, t7084: F) -> (F, F, F, F, F, F) {
    let t31302 = F::cast_from(3.0_f64) * t22574 * t31300;
    let t31304 = t7166 * t191 * t192;
    let t31305 = t31304 * t2020;
    let t31306 = t8607 * t6997;
    let t31310 = t8562 * t865;
    let t31311 = t2718 * t31310;
    let t31315 = t7084 * t225 * t258;
    (t31302, t31304, t31305, t31306, t31311, t31315)
}
