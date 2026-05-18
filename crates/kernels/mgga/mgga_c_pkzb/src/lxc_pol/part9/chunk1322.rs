//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1322/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1322<F: Float>(t23388: F, t1238: F, t179: F, t19193: F, t19196: F, t19206: F, t22260: F, t23367: F, t23375: F, t23382: F, t23383: F, t404: F, t6369: F, t6395: F, t8319: F, t932: F) -> F {
    let t23389 = F::new(0.14291339372689912324e-3) * t23388;
    let t23390 = t23367 + F::new(0.57165357490759649295e-3) * t19193 - F::new(0.85748036236139473944e-3) * t19196 - F::new(0.20579528696673473746e-1) * t8319 * t6369 - F::new(0.34299214494455789578e-2) * t19206 - F::new(0.85748036236139473944e-3) * t23375 - F::new(0.42874018118069736972e-3) * t404 * t179 * t932 * t22260 - t23382 + F::new(0.45732285992607719436e-2) * t23383 + F::new(0.22866142996303859718e-2) * t1238 * t6395 - t23389;
    t23390
}
