//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1001/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1001(t38486: f64, t901: f64, t13792: f64, t4379: f64, t12000: f64, t1429: f64, t2365: f64, t2366: f64, t47953: f64, t6963: f64, t6964: f64, t13801: f64, t1641: f64) -> (f64, f64, f64, f64, f64) {
    let t47978 = t38486 * t901;
    let t47980 = t4379 * t13792;
    let t47984 = t1429 * t2365 * t2366 * t12000;
    let t47987 = t6963 * t6964 * t47953;
    let t47989 = t1641 * t13801;
    (t47978, t47980, t47984, t47987, t47989)
}
