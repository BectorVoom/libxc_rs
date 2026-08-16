//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 814/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk814(t1217: f64, t21930: f64, t5206: f64, t5304: f64, t1091: f64, t5225: f64, t10248: f64, t446: f64, t1212: f64, t4969: f64, t2665: f64, t10270: f64, t21181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21931 = t21930 * t1217;
    let t21933 = t5206 * t5304;
    let t21945 = t1091 * t5225;
    let t21946 = t10248 * t21945;
    let t21947 = t446 * t21946;
    let t21949 = t4969 * t1212;
    let t21950 = t2665 * t21949;
    let t21951 = t446 * t21950;
    let t21953 = t10270 * t21181;
    (t21931, t21933, t21945, t21946, t21947, t21949, t21950, t21951, t21953)
}
