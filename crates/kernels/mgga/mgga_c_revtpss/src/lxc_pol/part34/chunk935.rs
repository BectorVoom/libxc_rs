//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 935/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk935<F: Float>(t1668: F, t3302: F, t357: F, t19572: F, t4982: F, t6299: F, t4893: F, t12168: F, t24078: F, t1651: F, t19556: F, t1089: F, t1678: F, t23820: F, t378: F, t6305: F) -> (F, F, F, F, F, F, F, F) {
    let t24083 = t3302 * t1668 * t357;
    let t24084 = t19572 * t24083;
    let t24089 = t4982 * t6299;
    let t24090 = t4893 * t24089;
    let t24093 = t24078 * t12168;
    let t24098 = t19556 * t1651;
    let t24104 = t1678 * t6299 * t1089;
    let t24108 = t378 * t23820 * t1089;
    let t24111 = t1678 * t6305;
    (t24084, t24089, t24090, t24093, t24098, t24104, t24108, t24111)
}
