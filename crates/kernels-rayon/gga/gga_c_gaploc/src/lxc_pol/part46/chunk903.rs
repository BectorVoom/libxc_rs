//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 903/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk903(t42651: f64, t1358: f64, t23915: f64, t42195: f64, t3394: f64, t488: f64, t9060: f64, t10171: f64, t2317: f64, t6525: f64, t2321: f64, t34478: f64, t9074: f64) -> (f64, f64, f64, f64, f64) {
    let t42652 = 0.142275033178380748e-1_f64 * t42651;
    let t42655 = 0.18970004423784099732e-1_f64 * t1358 * t23915 * t42195;
    let t42659 = 0.31616674039640166221e-2_f64 * t1358 * t9060 * t3394 * t488;
    let t42661 = t6525 * t10171 * t2317;
    let t42664 = t9074 * t34478 * t2321;
    (t42652, t42655, t42659, t42661, t42664)
}
