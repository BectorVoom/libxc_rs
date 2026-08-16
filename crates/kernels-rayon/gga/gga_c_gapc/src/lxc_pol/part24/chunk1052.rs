//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1052/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1052(t190: f64, t5261: f64, t1045: f64, t505: f64, t13738: f64, t8676: f64, t21: f64, t3142: f64, t3712: f64, t8654: f64, t4043: f64, t1030: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t26312 = t5261 * t190;
    let t26331 = t1045 * t505;
    let t26369 = t8676 * t13738;
    let t26396 = t3712 * t3142 * t21;
    let t26416 = t8654 * pi;
    let t26447 = t4043 * pi;
    let t26561 = t1030 * t26312;
    (t26312, t26331, t26369, t26396, t26416, t26447, t26561)
}
