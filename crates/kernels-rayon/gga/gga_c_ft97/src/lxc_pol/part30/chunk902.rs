//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 902/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk902(t665: f64, t7640: f64, t2344: f64, t2680: f64, t10491: f64, t863: f64, t192: f64, t33828: f64, t10696: f64, t2749: f64, t2770: f64, t2843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43912 = t665 * t7640;
    let t43917 = t2344 * t2680;
    let t44030 = t10491 * t863;
    let t44280 = t192 * t33828;
    let t44351 = t863 * t10696;
    let t44369 = t2770 * t2749;
    let t44523 = t2770 * t2843;
    (t43912, t43917, t44030, t44280, t44351, t44369, t44523)
}
