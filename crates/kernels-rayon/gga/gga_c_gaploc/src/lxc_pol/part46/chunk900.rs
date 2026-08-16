//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 900/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk900(t39677: f64, t39679: f64, t39681: f64, t12837: f64, t6305: f64, t2268: f64, t2440: f64, t3340: f64, t10223: f64, t894: f64, t1063: f64, t12814: f64, t12971: f64, t42580: f64, t42582: f64, t42584: f64, t42588: f64, t42591: f64, t42594: f64, t42597: f64, t42601: f64, t42602: f64, t42603: f64, t448: f64, t535: f64) -> f64 {
    let t42604 = 0.23712505529730124666e-2_f64 * t39677;
    let t42605 = 0.47425011059460249332e-2_f64 * t39679;
    let t42606 = 0.71137516589190373998e-2_f64 * t39681;
    let t42607 = t6305 * t12837;
    let t42610 = t2268 * t2440 * t3340;
    let t42613 = t2268 * t894 * t10223;
    let t42621 = -t42580 + t42582 + 0.47425011059460249332e-2_f64 * t42584 - t42588 - t42591 - t42594 + t42597 + t42601 + t42602 - t42603 + t42604 + t42605 - t42606 + 0.56910013271352299198e-1_f64 * t42607 + 0.56910013271352299198e-1_f64 * t42610 + 0.56910013271352299198e-1_f64 * t42613 - 0.28455006635676149599e-1_f64 * t1063 * t12971 * t448 + 0.28455006635676149599e-1_f64 * t2268 * t535 * t12814;
    t42621
}
