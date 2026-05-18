//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1098/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1098<F: Float>(t507: F, t7450: F, t7451: F, t7447: F, t8813: F, t8817: F, t7440: F, t8820: F, t2274: F, t30307: F, t137: F, t336: F, t4876: F, t578: F) -> (F, F, F, F, F, F) {
    let t35068 = t7450 * t507 * t7451;
    let t35070 = t7447 * t8813;
    let t35071 = F::new(0.84046875e-1) * t35070;
    let t35072 = t7447 * t8817;
    let t35073 = F::new(0.84046875e-1) * t35072;
    let t35074 = t7440 * t8820;
    let t35075 = F::new(0.5603125e-1) * t35074;
    let t35076 = t30307 * t2274;
    let t35080 = t578 * t336 * t4876 * t137;
    (t35068, t35071, t35073, t35075, t35076, t35080)
}
