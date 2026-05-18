//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 372/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk372<F: Float>(t2245: F, t2257: F, t2260: F, t2249: F, t2251: F) -> (F, F) {
    let t2264 = -F::new(0.34752604166666666667e-3) * t2257 * t2260 + F::new(0.17411041666666666666e-2) * t2245;
    let t2268 = F::new(0.9375e-1) * t2249 - F::new(0.20234375e-1) * t2251;
    (t2264, t2268)
}
