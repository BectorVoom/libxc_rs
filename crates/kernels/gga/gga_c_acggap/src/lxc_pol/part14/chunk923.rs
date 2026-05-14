//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 923/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk923<F: Float>(t36299: F, t30159: F, t36213: F, t7586: F, t2299: F, t7630: F, t1413: F, t7712: F, t2310: F, t30248: F, t542: F, t1967: F, t8855: F, t31773: F, t8916: F, t7447: F, t8920: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36300 = 0.14291339372689912324e-2 * t36299;
    let t36302 = t30159 * t7586 * t36213;
    let t36303 = 0.85748036236139473944e-3 * t36302;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36332 = 0.85748036236139473944e-3 * t36331;
    let t36333 = t7630 * t2310;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    let t36352 = 0.12862205435420921092e-2 * t36351;
    let t36353 = t31773 * t8916;
    let t36354 = 0.3361875e0 * t36353;
    let t36355 = t7447 * t8920;
    (t36300, t36303, t36327, t36332, t36333, t36349, t36352, t36354, t36355)
}
