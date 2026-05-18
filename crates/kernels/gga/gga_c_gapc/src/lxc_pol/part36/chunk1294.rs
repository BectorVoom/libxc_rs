//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1294/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1294<F: Float>(t3622: F, t2468: F, t3879: F, t10794: F, t11039: F, t12466: F, t12476: F, t12479: F, t12570: F, t23723: F, t2469: F, t2470: F, t2822: F, t31754: F, t31783: F, t338: F, t3565: F, t3568: F, t37356: F, t37478: F, t37510: F, t37524: F, t37539: F, t37554: F, t37570: F, t37584: F, t37599: F, t37614: F, t3883: F, t7056: F, t7063: F, t972: F) -> F {
    let t37619 = t3622 * t3622;
    let t37622 = t3879 * t2468;
    let t37642 = (t37510 + t37524 + t37539 + t37554 + t37570 + t37584 + t37599 + t37614) * t338 - t37356 + F::new(4.0) * t2469 * t37619 - t37478 + F::new(2.0) * t37622 * t2470 - F::new(12.0) * t23723 * t12476 + F::new(8.0) * t7056 * t12479 - F::new(2.0) * t3565 * t11039 + F::new(8.0) * t31754 * t3568 - F::new(6.0) * t7063 * t3883 * t2822 + F::new(4.0) * t2469 * t12570 * t972 - t12466 * t2822 - F::new(12.0) * t31783 * t10794;
    t37642
}
