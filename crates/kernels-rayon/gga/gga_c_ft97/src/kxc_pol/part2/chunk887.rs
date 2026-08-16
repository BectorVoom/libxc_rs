//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 887/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk887(t2514: f64, t3902: f64, t91: f64, t1154: f64, t2476: f64, t9890: f64, t2475: f64, t3938: f64, t747: f64, t13378: f64, t2354: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t13764 = t91 * t3902 * t2514;
    let t13768 = t91 * t9890 * t1154 * t2476;
    let t13770 = t2475 * t3938;
    let t13772 = t91 * t13770 * t747;
    let t13774 = t2354 * t13378;
    let t13775 = t446 * t13774;
    (t13764, t13768, t13772, t13775)
}
