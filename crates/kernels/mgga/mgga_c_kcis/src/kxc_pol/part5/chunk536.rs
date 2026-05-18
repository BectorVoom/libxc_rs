//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 536/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk536<F: Float>(t187: F, t5: F, t2302: F, t2306: F, t2313: F, t2336: F, t2344: F, t2346: F, t2350: F, t2357: F, t2368: F, t2376: F, t2705: F, t864: F, t867: F) -> (F, F) {
    let t2709 = t187 * t5;
    let t2718 = -t2302 - t2306 - t2313 + t2336 + t2344 + t187 * t2705 + F::new(0.24415406715670879921e-3) * t864 * t2346 + F::new(0.10843580882781524214e-1) * t2709 * t2350 + F::new(0.11696446794910408142e1) * t867 * t2357 - F::new(0.58482233974552040708e0) * t867 * t2368 - F::new(0.17315755899375863299e2) * t867 * t2376;
    (t2709, t2718)
}
