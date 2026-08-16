//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 651/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk651(t17944: f64, t3758: f64, t2427: f64, t25: f64, t13473: f64, t1113: f64, t200: f64, t237: f64, t213: f64, t5001: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17945 = t3758 * t17944;
    let t17957 = t2427 * t25;
    let t17964 = t3758 * t13473;
    let t17965 = t200 * t1113;
    let t17970 = t237 * t25;
    let t17971 = t3758 * t17970;
    let t17975 = t213 * t5001;
    (t17945, t17957, t17964, t17965, t17971, t17975)
}
