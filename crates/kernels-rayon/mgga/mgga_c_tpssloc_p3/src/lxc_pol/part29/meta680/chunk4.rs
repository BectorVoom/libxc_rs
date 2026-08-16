//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2290/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290(t1751: f64, t24594: f64, t24574: f64, t27403: f64, t1238: f64, t1251: f64, t14706: f64, t15425: f64, t15786: f64, t1716: f64, t2144: f64, t2154: f64, t2155: f64, t24596: f64, t24638: f64, t24880: f64, t24893: f64, t27741: f64, t3598: f64, t4930: f64, t498: f64, t5060: f64, t5089: f64, t51925: f64, t7283: f64, t7285: f64, t7286: f64, t85688: f64, t86451: f64, t86456: f64) -> f64 {
    let t94754 = t24594 * t1751;
    let t94759 = 0.54831135561607547884e-2_f64 * t24574 * t27403;
    let t94770 = 2.0_f64 * t1238 * t3598 * t2154 * t15786 + t15425 * t2144 * t498 - 2.0_f64 * t24893 * t5089 + 4.0_f64 * t1238 * t3598 * t27741 * t1251 - 2.0_f64 * t51925 * t2155 + 0.16449340668482264365e-1_f64 * t7283 * t4930 * t24638 + 0.36554090374405031923e-2_f64 * t7283 * t94754 * t24596 - t94759 + t86451 - 0.91385225936012579807e-3_f64 * t86456 + 4.0_f64 * t24880 * t5060 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t85688 - 0.27415567780803773942e-2_f64 * t7283 * t7285 * t7286 * t14706;
    t94770
}
