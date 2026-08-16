//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 799/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk799(t33308: f64, t9805: f64, t9806: f64, t15499: f64, t28640: f64, t3487: f64, t2963: f64, t3295: f64, t9796: f64, t1029: f64, t9829: f64, t20671: f64, t28069: f64, t33148: f64) -> (f64, f64, f64, f64, f64) {
    let t43403 = t9805 * t33308 * t9806;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43412 = t9796 * t2963 * t3295;
    let t43416 = t9796 * t1029 * t9829;
    let t43425 = t28069 * t20671 * t33148;
    (t43403, t43407, t43412, t43416, t43425)
}
