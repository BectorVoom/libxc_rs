//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1174/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1174(t23609: f64, t3327: f64, t10546: f64, t31548: f64, t4807: f64, t23741: f64, t3347: f64, t2268: f64, t26938: f64, t6767: f64, t21389: f64, t7937: f64) -> (f64, f64, f64, f64, f64) {
    let t31805 = 0.12646669615856066488e-1_f64 * t23609 * t3327;
    let t31811 = 0.39837009289946609438e0_f64 * t31548 * t10546 * t4807;
    let t31825 = 0.85365019907028448797e-1_f64 * t23741 * t3347;
    let t31835 = 0.68292015925622759036e0_f64 * t2268 * t26938 * t6767;
    let t31838 = 0.68292015925622759036e0_f64 * t2268 * t7937 * t21389;
    (t31805, t31811, t31825, t31835, t31838)
}
