//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 728/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk728<F: Float>(t218: F, t220: F, t5555: F, t1878: F, t679: F, t1478: F, t154: F, t277: F, t276: F, t2045: F, t735: F, t2065: F, t771: F) -> (F, F, F, F, F, F, F) {
    let t5557 = t218 * t5555 * t220;
    let t5558 = F::new(0.36793333333333333333e0) * t5557;
    let t5560 = t218 * t1878 * t679;
    let t5589 = t154 * t1478 * t277;
    let t5591 = F::new(5.0) / F::new(1296.0) * t276 * t5589;
    let t5597 = t735 * t2045;
    let t5609 = t771 * t2065;
    (t5557, t5558, t5560, t5589, t5591, t5597, t5609)
}
