//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1244/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1244(t20817: f64, t236: f64, t233: f64, t1881: f64, t5408: f64, t1876: f64, t4534: f64, t5411: f64, t13003: f64, t6272: f64, t2629: f64, t6276: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20818 = t236 * t20817;
    let t20819 = t233 * t20818;
    let t20821 = t1881 * t5408;
    let t20823 = t4534 * t1876;
    let t20824 = t233 * t20823;
    let t20826 = t1881 * t5411;
    let t20828 = t13003 * t6272;
    let t20833 = t2629 * t6276;
    (t20819, t20821, t20824, t20826, t20828, t20833)
}
