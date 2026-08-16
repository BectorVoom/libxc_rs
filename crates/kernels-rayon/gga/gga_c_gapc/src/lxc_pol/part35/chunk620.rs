//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 620/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk620(t1743: f64, t3717: f64, t1912: f64, t3666: f64, t3671: f64, t3676: f64, t3681: f64, t3685: f64, t3689: f64, t3692: f64, t3704: f64, t3710: f64, t3715: f64) -> (f64, f64) {
    let t3718 = t1743 * t3717;
    let t3719 = t3718 * t1912;
    let t3721 = 0.20241536458333333334e-4_f64 * t3666 - 0.17376185052903442709e-3_f64 * t3671 - 0.12650960286458333334e-5_f64 * t3676 + 0.10860115658064651693e-4_f64 * t3681 - 0.11594181388521408695e-4_f64 * t3685 - 0.33765185592488808582e-6_f64 * t3689 + 0.28985453471303521737e-5_f64 * t3692 - 0.24583187891642252608e-8_f64 * t3704 + 0.10551620497652752682e-7_f64 * t3710 + 0.33148893438893365995e-7_f64 * t3715 - 0.45289771048911752714e-7_f64 * t3719;
    (t3718, t3721)
}
