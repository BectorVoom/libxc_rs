//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 737/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk737(t531: f64, t6474: f64, t2349: f64, t590: f64, t1339: f64, t2293: f64, t107: f64, t6514: f64, t544: f64, t1421: f64, t2389: f64, t4494: f64, t901: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6937 = t531 * t6474;
    let t6944 = t2349 * t590;
    let t6949 = t1339 * t2293;
    let t6950 = t6949 * t590;
    let t6953 = t6514 * t107;
    let t6954 = t544 * t6953;
    let t6957 = t1421 * t2389;
    let t6959 = t4494 * t901;
    (t6937, t6944, t6950, t6953, t6954, t6957, t6959)
}
