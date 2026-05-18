//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 371/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk371<F: Float>(t1646: F, t2372: F, t1653: F, t1659: F, t2364: F, t26: F, t1651: F, t1658: F, t2366: F, t1664: F, t1645: F, t1668: F) -> (F, F, F, F, F, F, F, F) {
    let t2373 = t1646 * t2372;
    let t2376 = t1653 * t2372;
    let t2378 = t1659 * t2364;
    let t2379 = t26 * t2378;
    let t2381 = F::new(0.1898925e1) * t2373 - t1651 - F::new(0.29896666666666666667e0) * t2366 + F::new(0.3071625e0) * t2376 - t1658 - F::new(0.82156666666666666667e-1) * t2379;
    let t2382 = t2381 * t1664;
    let t2384 = F::new(1.0) * t1645 * t2382;
    let t2386 = -t1668 - F::new(0.92708333333333333333e-2) * t2366;
    (t2373, t2376, t2378, t2379, t2381, t2382, t2384, t2386)
}
