//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2050/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2050(t25580: f64, t3053: f64, t23529: f64, t4571: f64, t13961: f64, t6755: f64, t14202: f64, t6765: f64, t13950: f64, t23422: f64, t4603: f64, t14159: f64, t6717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88305 = t25580 * t3053 / 1728.0_f64;
    let t88307 = t23529 * t4571 / 324.0_f64;
    let t88320 = t6755 * t13961 / 1152.0_f64;
    let t88321 = t6765 * t14202;
    let t88324 = t6765 * t13950 / 1728.0_f64;
    let t88335 = t23422 * t4603 / 162.0_f64;
    let t88336 = t6717 * t14159;
    (t88305, t88307, t88320, t88321, t88324, t88335, t88336)
}
