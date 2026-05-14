//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1070/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1070<F: Float>(t14047: F, t6347: F, t16899: F, t5928: F, t14056: F, t5932: F, t1165: F, t12945: F, t13221: F, t13226: F, t13229: F, t16720: F, t16724: F, t16728: F, t16730: F, t16739: F, t1884: F, t945: F) -> (F,) {
    let t21651 = t14047 * t6347;
    let t21657 = t16899 * t5928;
    let t21659 = t14056 * t5932;
    let t21661 = 0.34013387707001991333e-1 * t13221 - t13226 - t13229 + 0.68598428988911579156e-2 * t16720 + 0.34299214494455789578e-2 * t16724 + 0.68598428988911579156e-2 * t16728 - 0.85748036236139473944e-3 * t16730 + 35.0 / 54.0 * t16739 + 0.68598428988911579156e-2 * t21651 + 0.85748036236139473944e-2 * t12945 * t1165 * t1884 * t945 + 0.34299214494455789578e-1 * t21657 + 0.13719685797782315831e-1 * t21659;
    (t21661,)
}
