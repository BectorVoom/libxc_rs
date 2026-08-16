//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 858/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk858(t1317: f64, t4756: f64, t201: f64, t3318: f64, t104: f64, t16310: f64, t16315: f64, t16318: f64, t16319: f64, t16572: f64, t3316: f64, t3539: f64, t6359: f64, t6437: f64, t6449: f64, t6709: f64, t6711: f64, t6766: f64, t714: f64, t95: f64) -> (f64, f64) {
    let t16577 = t4756 * t1317;
    let t16578 = t16577 * t201;
    let t16579 = t3318 * t16578;
    let t16582 = -t6709 + t6359 + 0.51689762869806860992e-2_f64 * t95 * t104 * t16310 * t6766 + 0.46520786582826174894e-1_f64 * t3539 * t16315 + t6711 + t16318 - t16319 + 0.25844881434903430496e-2_f64 * t95 * t104 * t16572 * t714 + 3.0_f64 / 2.0_f64 * t3316 * t16579 - t6437 + t6449;
    (t16579, t16582)
}
