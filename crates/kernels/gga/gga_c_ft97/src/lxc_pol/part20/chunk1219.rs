//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1219/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1219<F: Float>(t10491: F, t1476: F, t14690: F, t1901: F, t43917: F, t14686: F, t10683: F, t15425: F, t6317: F, t6318: F, t25165: F, t28735: F, t28736: F, t840: F, t4226: F, t856: F) -> (F, F, F, F, F) {
    let t113076 = t10491 * t1476;
    let t113078 = t1901 * t113076 * t14690;
    let t113080 = t43917 * t1476;
    let t113082 = t1901 * t113080 * t14686;
    let t113086 = t6317 * t10683 * t6318 * t15425;
    let t113090 = t28735 * t840 * t25165 * t28736;
    let t113095 = t28735 * t840 * t6318 * t4226 * t856;
    (t113078, t113082, t113086, t113090, t113095)
}
