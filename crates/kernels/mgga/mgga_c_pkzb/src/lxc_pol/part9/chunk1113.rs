//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1113/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1113<F: Float>(t18509: F, t369: F, t6287: F, t858: F, t2277: F, t356: F, t2280: F, t2099: F, t3235: F, t6386: F, t2387: F, t824: F) -> (F, F, F, F, F, F) {
    let t18878 = t369 * t18509;
    let t18882 = t858 * t6287;
    let t18885 = t2277 * t2277;
    let t18887 = t356 / t18885;
    let t18888 = t2280 * t2280;
    let t18889 = F::new(1.0) / t18888;
    let t18940 = t3235 * t2099 * t6386;
    let t18957 = t824 * t2387;
    (t18878, t18882, t18887, t18889, t18940, t18957)
}
