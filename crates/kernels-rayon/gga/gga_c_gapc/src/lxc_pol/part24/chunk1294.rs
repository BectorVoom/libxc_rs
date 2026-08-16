//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1294/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1294(t3622: f64, t2468: f64, t3879: f64, t10794: f64, t11039: f64, t12466: f64, t12476: f64, t12479: f64, t12570: f64, t23723: f64, t2469: f64, t2470: f64, t2822: f64, t31754: f64, t31783: f64, t338: f64, t3565: f64, t3568: f64, t37356: f64, t37478: f64, t37510: f64, t37524: f64, t37539: f64, t37554: f64, t37570: f64, t37584: f64, t37599: f64, t37614: f64, t3883: f64, t7056: f64, t7063: f64, t972: f64) -> f64 {
    let t37619 = t3622 * t3622;
    let t37622 = t3879 * t2468;
    let t37642 = (t37510 + t37524 + t37539 + t37554 + t37570 + t37584 + t37599 + t37614) * t338 - t37356 + 4.0_f64 * t2469 * t37619 - t37478 + 2.0_f64 * t37622 * t2470 - 12.0_f64 * t23723 * t12476 + 8.0_f64 * t7056 * t12479 - 2.0_f64 * t3565 * t11039 + 8.0_f64 * t31754 * t3568 - 6.0_f64 * t7063 * t3883 * t2822 + 4.0_f64 * t2469 * t12570 * t972 - t12466 * t2822 - 12.0_f64 * t31783 * t10794;
    t37642
}
