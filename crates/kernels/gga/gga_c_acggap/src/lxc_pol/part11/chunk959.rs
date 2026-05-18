//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 959/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk959<F: Float>(t1967: F, t7523: F, t7535: F, t2104: F, t7610: F, t1988: F, t7472: F, t1113: F, t7736: F, t1098: F, t7605: F, t3445: F, t7647: F) -> (F, F, F, F, F, F, F) {
    let t31845 = t1967 * t7523;
    let t31847 = t1967 * t7535;
    let t31849 = t7610 * t2104;
    let t31851 = t1988 * t7472;
    let t31855 = t7736 * t1113;
    let t31857 = t7605 * t1098;
    let t31859 = t7647 * t3445;
    (t31845, t31847, t31849, t31851, t31855, t31857, t31859)
}
