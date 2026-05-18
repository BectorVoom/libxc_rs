//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 910/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk910<F: Float>(t30904: F, t1035: F, t1039: F, t7613: F, t1200: F, t7605: F, t1988: F, t7535: F, t30589: F, t7548: F, t2109: F, t7630: F) -> (F, F, F, F, F, F) {
    let t30905 = F::new(0.25724410870841842183e-2) * t30904;
    let t30907 = t1035 * t7613 * t1039;
    let t30908 = F::new(0.12004725073059526352e-1) * t30907;
    let t30916 = t7605 * t1200;
    let t30918 = t1988 * t7535;
    let t30920 = t30589 * t7548;
    let t30921 = F::new(0.41930789719472202756e-2) * t30920;
    let t30924 = t7630 * t2109;
    (t30905, t30908, t30916, t30918, t30921, t30924)
}
