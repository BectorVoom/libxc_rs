//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 813/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk813<F: Float>(t355: F, t864: F, t30572: F, t368: F, t7458: F, t7709: F, t7799: F, t1967: F, t7763: F, t7701: F, t381: F, t7636: F, t7461: F, t7637: F, t7770: F, t13716: F, t577: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30573 = t355 * t864;
    let t30576 = t30572 * t7458 * t368 * t30573;
    let t30577 = 0.42874018118069736972e-3 * t30576;
    let t30582 = t7799 * t7709;
    let t30584 = t1967 * t7763;
    let t30586 = t1967 * t7701;
    let t30589 = t381 * t7636;
    let t30590 = t30589 * t7461;
    let t30591 = 0.28582678745379824649e-2 * t30590;
    let t30592 = t7637 * t7770;
    let t30594 = t13716 * t577;
    (t30573, t30577, t30582, t30584, t30586, t30589, t30591, t30592, t30594)
}
