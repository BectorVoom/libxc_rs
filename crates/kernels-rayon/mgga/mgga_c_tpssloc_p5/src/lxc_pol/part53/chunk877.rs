//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 877/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk877(t31645: f64, t6888: f64, t6883: f64, t8622: f64, t22666: f64, t8621: f64, t1985: f64, t8612: f64, t225: f64, t8729: f64, t31320: f64, t798: f64, t8728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31662 = t6883 * t8612;
    let t31964 = t8729 * t225;
    let t31971 = 0.16449340668482264365e-1_f64 * t31320;
    let t31974 = t798 * t8728;
    (t31646, t31648, t31650, t31651, t31662, t31964, t31971, t31974)
}
