//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 966/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk966(t2268: f64, t24139: f64, t29975: f64, t8124: f64, t39695: f64, t6520: f64, t6525: f64, t7888: f64, t2326: f64, t3394: f64, t6514: f64, t9074: f64) -> (f64, f64, f64, f64) {
    let t42637 = 0.68292015925622759036e0_f64 * t2268 * t24139 * t8124 * t29975;
    let t42638 = 0.63233348079280332443e-2_f64 * t39695;
    let t42640 = t6525 * t7888 * t6520;
    let t42641 = 0.71137516589190373998e-2_f64 * t42640;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    (t42637, t42638, t42641, t42644)
}
