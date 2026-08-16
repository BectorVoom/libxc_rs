//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 614/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk614(t22952: f64, t25885: f64, t432: f64, t965: f64, t1871: f64, t5675: f64, t23008: f64, t92: f64, t473: f64, t452: f64, t2: f64, t6454: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25886 = t22952 * t25885;
    let t25888 = t965 * t432;
    let t25890 = t1871 * t5675 * t25888;
    let t25891 = t22952 * t25890;
    let t25893 = t23008 * t92;
    let t25894 = t965 * t473;
    let t25896 = t452 * t5675 * t25894;
    let t25897 = t25893 * t25896;
    let t25899 = t2 * t6454;
    (t25886, t25888, t25891, t25893, t25894, t25897, t25899)
}
