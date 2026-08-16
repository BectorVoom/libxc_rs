//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 618/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk618(t22993: f64, t920: f64, t1564: f64, t446: f64, t1882: f64, t6513: f64, t23054: f64, t6501: f64, t22980: f64, t22991: f64, t23016: f64, t23029: f64, t23114: f64, t25926: f64, t25931: f64, t25935: f64, t25940: f64) -> (f64, f64, f64, f64, f64) {
    let t25942 = t22993 * t920;
    let t25943 = t1564 * t25942;
    let t25944 = t446 * t25943;
    let t25946 = t1882 * t6513;
    let t25948 = t23054 * t6501;
    let t25952 = -t25926 / 9.0_f64 + t25931 / 27.0_f64 - t25935 / 9.0_f64 - t22980 / 9.0_f64 - t22991 / 27.0_f64 + t25940 / 9.0_f64 + t25944 / 9.0_f64 - t25946 / 27.0_f64 - t25948 / 54.0_f64 - t23016 / 36.0_f64 + t23029 / 18.0_f64 - t23114;
    (t25942, t25944, t25946, t25948, t25952)
}
