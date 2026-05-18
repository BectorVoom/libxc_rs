//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 961/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk961<F: Float>(t1181: F, t23745: F, t604: F, t7493: F, t31362: F, t8775: F, t30268: F, t8956: F, t1983: F, t30262: F, t7586: F, t8406: F) -> (F, F, F, F) {
    let t34099 = t7493 * t1181 * t604 * t23745;
    let t34100 = F::new(0.21437009059034868486e-2) * t34099;
    let t34101 = t31362 * t8775;
    let t34102 = F::new(0.10718504529517434243e-2) * t34101;
    let t34107 = t30268 * t8956;
    let t34127 = t30262 * t7586 * t1983 * t8406;
    (t34100, t34102, t34107, t34127)
}
