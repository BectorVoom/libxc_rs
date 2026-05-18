//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1392/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1392<F: Float>(t34642: F, t10514: F, t21370: F, t10531: F, t10534: F, t1406: F, t10557: F, t6795: F, t8072: F, t9285: F, t204: F, t2476: F, t34567: F, t34621: F, t34623: F, t34626: F, t34628: F, t34631: F, t34634: F, t34636: F, t34638: F, t34640: F) -> F {
    let t34643 = F::new(0.89376224879626066674e-1) * t34642;
    let t34645 = F::new(0.12423108009070322895e3) * t21370 * t10514;
    let t34648 = F::new(0.55213813373645879534e2) * t1406 * t10531 * t10534;
    let t34650 = F::new(0.42900587942220512003e1) * t10557 * t6795;
    let t34652 = F::new(0.71500979903700853338e0) * t9285 * t8072;
    let t34656 = t34621 + t34623 + t34626 + t34628 + t34631 + t34634 + t34636 + t34638 - t34640 - t34643 - t34645 + t34648 + t34650 + t34652 + F::new(0.92023022289409799224e1) * t2476 * t204 * t34567;
    t34656
}
