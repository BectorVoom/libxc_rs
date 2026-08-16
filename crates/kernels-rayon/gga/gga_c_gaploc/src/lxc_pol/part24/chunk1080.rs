//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1080/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1080(t2366: f64, t25574: f64, t1265: f64, t986: f64, t6508: f64, t1352: f64, t2755: f64, t158: f64, t7861: f64, t1328: f64, t20368: f64, t2754: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25575 = t2366 * t25574;
    let t25579 = t986 * t1265;
    let t25580 = t6508 * t25579;
    let t25665 = t2755 * t1352;
    let t25694 = t158 * t7861;
    let t25722 = t986 * t1328;
    let t25723 = t20368 * t25722;
    let t25729 = t2754 * t475;
    (t25575, t25579, t25580, t25665, t25694, t25722, t25723, t25729)
}
