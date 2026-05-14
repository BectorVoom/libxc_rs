//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 369/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk369<F: Float>(t2307: F, t14: F, t661: F, t662: F, t128: F) -> (F, F, F, F, F, F) {
    let t2308 = 1.0 / t2307;
    let t2309 = t14 * t2308;
    let t2310 = t661 * t661;
    let t2311 = t2310 * t662;
    let t2313 = 2.0 * t2309 * t2311;
    let t2314 = 1.0 / t128;
    (t2308, t2309, t2310, t2311, t2313, t2314)
}
