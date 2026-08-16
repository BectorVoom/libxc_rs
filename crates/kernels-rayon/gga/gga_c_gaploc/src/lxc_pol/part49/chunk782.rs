//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 782/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk782(t13728: f64, t2343: f64, t2268: f64, t11977: f64, t888: f64, t3691: f64, t894: f64, t11986: f64, t2325: f64, t883: f64, t882: f64, t12764: f64, t12769: f64, t12774: f64, t12802: f64, t12809: f64, t12836: f64, t12838: f64, t12842: f64, t13726: f64) -> (f64, f64, f64, f64, f64) {
    let t13729 = t2343 * t13728;
    let t13730 = t2268 * t13729;
    let t13732 = t11977 * t888;
    let t13733 = t2268 * t13732;
    let t13735 = t894 * t3691;
    let t13736 = t2268 * t13735;
    let t13740 = t2325 * t883 * t11986;
    let t13741 = t882 * t13740;
    let t13745 = 0.11856252764865062333e-2_f64 * t13726 + 0.56910013271352299198e-1_f64 * t13730 - 0.85365019907028448797e-1_f64 * t13733 + 0.28455006635676149599e-1_f64 * t13736 + t12836 + 0.28455006635676149599e-1_f64 * t12838 - t12842 - 0.11856252764865062333e-2_f64 * t13741 + 0.56910013271352299198e-1_f64 * t12764 + t12769 - 0.85365019907028448797e-1_f64 * t12774 - t12802 - t12809;
    (t13729, t13732, t13735, t13740, t13745)
}
