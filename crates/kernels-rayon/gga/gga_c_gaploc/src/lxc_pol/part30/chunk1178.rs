//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1178/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1178(t197: f64, t3338: f64, t161: f64, t25893: f64, t6520: f64, t23763: f64, t10215: f64, t158: f64, t475: f64, t6508: f64, t25722: f64, t4261: f64, t9074: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31730 = t197 * t3338;
    let t31731 = t31730 * t161;
    let t31735 = t25893 * t6520;
    let t31737 = 0.18970004423784099733e-1_f64 * t23763 * t31735;
    let t31740 = t158 * t10215;
    let t31747 = t3338 * t475;
    let t31748 = t6508 * t31747;
    let t31752 = t6508 * t25722;
    let t31754 = t9074 * t4261 * t31752;
    (t31730, t31731, t31735, t31737, t31740, t31747, t31748, t31752, t31754)
}
