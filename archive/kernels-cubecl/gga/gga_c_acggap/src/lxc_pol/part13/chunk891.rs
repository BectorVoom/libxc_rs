//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 891/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk891<F: Float>(t2100: F, t30567: F, t2104: F, t7630: F, t1035: F, t1979: F, t355: F, t864: F, t368: F, t7458: F, t7709: F, t7799: F) -> (F, F, F, F, F, F) {
    let t30568 = t30567 * t2100;
    let t30569 = F::cast_from(0.56606566121287473723e-2_f64) * t30568;
    let t30570 = t7630 * t2104;
    let t30572 = t1035 * t1979;
    let t30573 = t355 * t864;
    let t30576 = t30572 * t7458 * t368 * t30573;
    let t30577 = F::cast_from(0.42874018118069736972e-3_f64) * t30576;
    let t30582 = t7799 * t7709;
    (t30569, t30570, t30572, t30573, t30577, t30582)
}
