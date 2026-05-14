//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 864/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk864<F: Float>(t30049: F, t7461: F, t1089: F, t1198: F, t2079: F, t2080: F, t1967: F, t7523: F, t7535: F, t2104: F, t7610: F, t1988: F, t7472: F, t1113: F, t7736: F, t1098: F, t7605: F) -> (F, F, F, F, F, F, F, F) {
    let t31839 = t30049 * t7461;
    let t31840 = 0.42874018118069736972e-3 * t31839;
    let t31843 = t2079 * t1089 * t1198 * t2080;
    let t31845 = t1967 * t7523;
    let t31847 = t1967 * t7535;
    let t31849 = t7610 * t2104;
    let t31851 = t1988 * t7472;
    let t31855 = t7736 * t1113;
    let t31857 = t7605 * t1098;
    (t31840, t31843, t31845, t31847, t31849, t31851, t31855, t31857)
}
