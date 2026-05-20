//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2573/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573<F: Float>(t57421: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F, t1789: F, t2434: F, t1012: F, t44958: F, t13026: F, t140: F) -> (F, F, F, F, F, F) {
    let t57422 = F::cast_from(0.5081365110289746604e-3_f64) * t57421;
    let t57463 = t1235 * t371 * t676 * t5318;
    let t57464 = F::cast_from(0.14291339372689912324e-3_f64) * t57463;
    let t57465 = t56331 * t225;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    (t57422, t57464, t57465, t57471, t57480, t57484)
}
