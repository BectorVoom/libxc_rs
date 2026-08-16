//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 996/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk996(t47803: f64, t6717: f64, t6914: f64, t12079: f64, t2389: f64, t12092: f64, t2482: f64, t9267: f64, t12000: f64, t123: f64, t883: f64, t2487: f64, t2488: f64) -> (f64, f64, f64, f64, f64) {
    let t47864 = t6914 * t6717 * t47803;
    let t47866 = t12079 * t2389;
    let t47869 = t9267 * t12092 * t2482;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    (t47864, t47866, t47869, t47877, t47879)
}
