//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2538/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538<F: Float>(t2873: F, t4587: F, t11298: F, t1596: F, t11466: F, t1633: F, t11299: F, t1609: F, t51913: F, t51915: F, t51973: F, t52035: F) -> (F, F, F, F, F, F, F, F) {
    let t52505 = t4587 * t2873;
    let t52508 = t1596 * t11298;
    let t52511 = t11466 * t1633;
    let t52514 = t11299 * t1609;
    let t52546 = F::cast_from(0.69463333333333333334e0_f64) * t51913;
    let t52547 = F::cast_from(0.11577222222222222222e0_f64) * t51915;
    let t52573 = F::cast_from(0.68863333333333333332e0_f64) * t51973;
    let t52597 = F::cast_from(0.13772666666666666666e1_f64) * t52035;
    (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597)
}
