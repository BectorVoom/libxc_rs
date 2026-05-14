//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 878/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk878<F: Float>(t32010: F, t7963: F, t8306: F, t16548: F, t7942: F, t2176: F, t3909: F, t633: F, t848: F, t464: F, t14575: F, t8111: F, t880: F, t32194: F, t3912: F, t2226: F, t32063: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33085 = t7963 * t8306 * t32010;
    let t33088 = t7942 * t8306 * t16548;
    let t33090 = t2176 * t3909;
    let t33092 = t848 * t633;
    let t33093 = t33092 * t464;
    let t33097 = t7942 * t8306 * t14575;
    let t33100 = 0.19756347548806534796e1 * t8111 * t880;
    let t33104 = t7963 * t8306 * t32194;
    let t33107 = 0.65854491829355115987e0 * t2176 * t3912;
    let t33110 = t32063 * t2226;
    (t33085, t33088, t33090, t33092, t33093, t33097, t33100, t33104, t33107, t33110)
}
