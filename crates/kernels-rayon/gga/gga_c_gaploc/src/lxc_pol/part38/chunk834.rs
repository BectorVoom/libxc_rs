//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 834/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk834(t2268: f64, t38184: f64, t888: f64, t2349: f64, t3565: f64, t11264: f64, t6767: f64, t3516: f64, t4538: f64, t6759: f64, t11254: f64, t2343: f64, t6509: f64) -> (f64, f64, f64, f64, f64) {
    let t44355 = 0.85365019907028448797e-1_f64 * t2268 * t38184 * t888;
    let t44358 = 0.19918504644973304719e0_f64 * t2268 * t3565 * t2349;
    let t44363 = 0.14227503317838074799e1_f64 * t2268 * t11264 * t6767;
    let t44364 = t4538 * t3516;
    let t44367 = 0.17073003981405689759e1_f64 * t2268 * t44364 * t6759;
    let t44371 = 0.34146007962811379518e0_f64 * t2268 * t2343 * t11254 * t6509;
    (t44355, t44358, t44363, t44367, t44371)
}
