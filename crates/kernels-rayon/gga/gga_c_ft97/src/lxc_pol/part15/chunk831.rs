//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 831/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk831(t21369: f64, t319: f64, t835: f64, t10758: f64, t21351: f64, t1255: f64, t4973: f64, t2857: f64, t4965: f64, t1212: f64, t4917: f64, t4265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22218 = t835 * t319 * t21369;
    let t22222 = t10758 * t319 * t21351;
    let t22226 = t835 * t1255 * t4973;
    let t22230 = t2857 * t1255 * t4965;
    let t22240 = t4917 * t1212;
    let t22241 = t4265 * t22240;
    (t22218, t22222, t22226, t22230, t22240, t22241)
}
