//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2511/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2511(t1130: f64, t12393: f64, t3376: f64, t3432: f64, t3488: f64, t3495: f64, t1175: f64, t12485: f64, t3444: f64, t3476: f64, t1156: f64, t12469: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45041 = t12393 * t1130;
    let t45046 = t3376 * t3432;
    let t45061 = t3488 * t3495;
    let t45064 = t1175 * t12485;
    let t45075 = t3444 * t3476;
    let t45080 = t1156 * t12469;
    (t45041, t45046, t45061, t45064, t45075, t45080)
}
