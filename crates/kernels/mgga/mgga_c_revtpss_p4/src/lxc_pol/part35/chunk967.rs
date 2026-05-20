//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 967/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk967<F: Float>(t12168: F, t24078: F, t1651: F, t19556: F, t1089: F, t1678: F, t6299: F, t23820: F, t378: F, t6305: F, t3304: F, t1668: F, t6343: F) -> (F, F, F, F, F, F, F) {
    let t24093 = t24078 * t12168;
    let t24098 = t19556 * t1651;
    let t24104 = t1678 * t6299 * t1089;
    let t24108 = t378 * t23820 * t1089;
    let t24111 = t1678 * t6305;
    let t24112 = t24111 * t3304;
    let t24116 = t6343 * t1668 * t1089;
    (t24093, t24098, t24104, t24108, t24111, t24112, t24116)
}
