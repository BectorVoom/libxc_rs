//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 286/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk286<F: Float>(t4041: F, t446: F, t18: F, t792: F, t3704: F, t89: F, t1213: F, t375: F, t1212: F, t668: F, t505: F, t2665: F, t2680: F, t824: F, t193: F, t284: F, t811: F) -> (F, F, F, F, F, F, F) {
    let t4042 = t446 * t4041;
    let t4044 = t792 * t18;
    let t4046 = t89 * t3704 * t4044;
    let t4049 = t89 * t375 * t1213;
    let t4051 = t1212 * t668;
    let t4052 = t4051 * t505;
    let t4053 = t2665 * t4052;
    let t4054 = t446 * t4053;
    let t4056 = t2680 * t1212;
    let t4057 = t4056 * t824;
    let t4059 = t89 * t193 * t4057;
    let t4061 = t811 * t284;
    (t4042, t4046, t4049, t4052, t4054, t4059, t4061)
}
