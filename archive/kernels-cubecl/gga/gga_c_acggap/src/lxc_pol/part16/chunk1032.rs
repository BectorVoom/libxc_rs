//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1032/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1032<F: Float>(t36299: F, t30159: F, t36213: F, t7586: F, t2299: F, t7630: F, t1413: F, t7712: F, t2310: F, t30248: F, t542: F, t1967: F, t8855: F) -> (F, F, F, F, F, F, F) {
    let t36300 = F::cast_from(0.14291339372689912324e-2_f64) * t36299;
    let t36302 = t30159 * t7586 * t36213;
    let t36303 = F::cast_from(0.85748036236139473944e-3_f64) * t36302;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36332 = F::cast_from(0.85748036236139473944e-3_f64) * t36331;
    let t36333 = t7630 * t2310;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    (t36300, t36303, t36327, t36332, t36333, t36349, t36351)
}
