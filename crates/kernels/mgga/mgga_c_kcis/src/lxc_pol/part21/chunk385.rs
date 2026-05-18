//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 385/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk385<F: Float>(t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F, t662: F, t646: F, t644: F, t14: F, t31: F, t2310: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2333 = -F::new(0.42198333333333333333e0) * t2318 + F::new(0.84396666666666666666e0) * t2321 + F::new(0.39862222222222222223e0) * t2323 + F::new(0.68258333333333333333e-1) * t2327 + F::new(0.13651666666666666667e0) * t2329 + F::new(0.13692777777777777778e0) * t2331;
    let t2334 = t2333 * t662;
    let t2336 = F::new(1.0) * t646 * t2334;
    let t2337 = t644 * t644;
    let t2338 = F::new(1.0) / t2337;
    let t2339 = t14 * t2338;
    let t2340 = t31 * t31;
    let t2341 = F::new(1.0) / t2340;
    let t2342 = t2310 * t2341;
    (t2333, t2334, t2336, t2337, t2338, t2339, t2340, t2341, t2342)
}
