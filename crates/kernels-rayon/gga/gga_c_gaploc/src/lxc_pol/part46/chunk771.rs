//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 771/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk771(t1: f64, t1559: f64, t544: f64, t986: f64, t10241: f64, t1359: f64, t12380: f64, t455: f64, t145: f64, t459: f64, t12385: f64, t2281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35204 = t544 * t1559 * t986 * t1;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t39622 = t12380 * t455;
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    (t35204, t35215, t35216, t39622, t39624, t39626)
}
