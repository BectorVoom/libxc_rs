//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 829/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk829(t40165: f64, t44285: f64, t9074: f64, t13401: f64, t1358: f64, t2299: f64, t488: f64, t42579: f64, t3529: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t44287 = t9074 * t44285 * t40165;
    let t44288 = 0.142275033178380748e-1_f64 * t44287;
    let t44292 = 0.31616674039640166221e-2_f64 * t1358 * t2299 * t13401 * t488;
    let t44293 = 0.47425011059460249332e-2_f64 * t42579;
    let t44294 = t3529 * t874;
    (t44288, t44292, t44293, t44294)
}
