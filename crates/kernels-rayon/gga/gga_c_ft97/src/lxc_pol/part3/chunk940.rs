//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 940/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk940(t18514: f64, t3892: f64, t2606: f64, t1168: f64, t18: f64, t2607: f64, t1131: f64, t2600: f64, t2599: f64, t5171: f64, t9787: f64, t3972: f64, t992: f64) -> (f64, f64, f64, f64, f64) {
    let t18515 = t3892 * t18514;
    let t18516 = t2606 * t18515;
    let t18519 = t18 * t1168;
    let t18520 = t2607 * t18519;
    let t18521 = t2606 * t18520;
    let t18524 = t18 * t1131;
    let t18525 = t2600 * t18524;
    let t18526 = t2599 * t18525;
    let t18529 = t9787 * t5171;
    let t18532 = t992 * t3972;
    (t18516, t18521, t18526, t18529, t18532)
}
