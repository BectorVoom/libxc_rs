//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1037/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1037(t10007: f64, t5397: f64, t15478: f64, t1964: f64, t10012: f64, t169: f64, t5750: f64, t1234: f64, t1683: f64, t5335: f64, t5344: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15483 = t10007 * t5397;
    let t15488 = t1964 * t15478;
    let t15490 = t10012 * t5397;
    let t15499 = t169 * t5750;
    let t15660 = t1234 * t1234;
    let t15665 = 1.0_f64 / t5335 / t1683;
    let t15667 = t15665 * t92 * t5344;
    (t15483, t15488, t15490, t15499, t15660, t15665, t15667)
}
