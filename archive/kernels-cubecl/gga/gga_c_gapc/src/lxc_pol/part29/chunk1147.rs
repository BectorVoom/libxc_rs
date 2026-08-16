//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1147/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1147<F: Float>(t11905: F, t15491: F, t18107: F, t33149: F, t10063: F, t11930: F, t11597: F, t3363: F, t3415: F, t11902: F, t16296: F, t18018: F) -> (F, F, F, F, F, F) {
    let t34166 = t11905 * t15491;
    let t34169 = t33149 * t18107;
    let t34171 = t11930 * t10063;
    let t34174 = t3363 * t11597 * t3415;
    let t34176 = t11902 * t16296;
    let t34178 = t11905 * t18018;
    (t34166, t34169, t34171, t34174, t34176, t34178)
}
