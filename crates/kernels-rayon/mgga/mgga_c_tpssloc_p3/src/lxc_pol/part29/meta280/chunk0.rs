//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1291/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1291(t1653: f64, t7363: f64, t7362: f64, t1716: f64, t2148: f64, t1755: f64, t7376: f64, t7375: f64, t1751: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8066 = t7363 * t1653;
    let t8067 = t7362 * t8066;
    let t8070 = t1716 * t2148;
    let t8073 = t1755 * t7376;
    let t8074 = t7375 * t8073;
    let t8077 = t2147 * t1751;
    (t8066, t8067, t8070, t8073, t8074, t8077)
}
