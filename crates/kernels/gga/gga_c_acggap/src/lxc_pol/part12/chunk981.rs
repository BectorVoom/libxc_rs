//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 981/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk981<F: Float>(t633: F, t848: F, t464: F, t14575: F, t7942: F, t8306: F, t8111: F, t880: F, t32194: F, t7963: F, t2176: F, t3912: F) -> (F, F, F, F, F, F) {
    let t33092 = t848 * t633;
    let t33093 = t33092 * t464;
    let t33097 = t7942 * t8306 * t14575;
    let t33100 = F::cast_from(0.19756347548806534796e1_f64) * t8111 * t880;
    let t33104 = t7963 * t8306 * t32194;
    let t33107 = F::cast_from(0.65854491829355115987e0_f64) * t2176 * t3912;
    (t33092, t33093, t33097, t33100, t33104, t33107)
}
