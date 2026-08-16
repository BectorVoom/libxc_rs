//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 478/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk478(t381: f64, t5848: f64, t1603: f64, t1625: f64, t1044: f64, t248: f64, t5685: f64, t3062: f64, t5677: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64) -> (f64, f64, f64, f64, f64) {
    let t5849 = t5848 * t381;
    let t5851 = t1603 * t1625;
    let t5857 = t248 * t1044 * t5685;
    let t5861 = t248 * t3062 * t5677;
    let t5866 = -t5691 + t5693 - t5697 + t5729 + t5732 + t5798 + t5800 - t5802 + t5806 - t5810 - t5814;
    (t5849, t5851, t5857, t5861, t5866)
}
