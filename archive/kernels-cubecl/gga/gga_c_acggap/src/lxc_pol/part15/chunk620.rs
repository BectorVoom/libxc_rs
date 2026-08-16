//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 620/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk620<F: Float>(t1323: F, t4593: F, t1327: F, t1314: F, t1318: F, t1814: F, t435: F, t1165: F, t3544: F, t1444: F, t530: F, t1181: F) -> (F, F, F, F, F, F, F) {
    let t5910 = t4593 * t1323;
    let t5913 = t4593 * t1327;
    let t5916 = t4593 * t1314;
    let t5919 = t4593 * t1318;
    let t5922 = t435 * t1814;
    let t5924 = t1165 * t5922 * t3544;
    let t5927 = t530 * t1444;
    let t5928 = t1181 * t5927;
    (t5910, t5913, t5916, t5919, t5922, t5924, t5928)
}
