//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1174/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1174<F: Float>(t23609: F, t3327: F, t10546: F, t31548: F, t4807: F, t23741: F, t3347: F, t2268: F, t26938: F, t6767: F, t21389: F, t7937: F) -> (F, F, F, F, F) {
    let t31805 = F::cast_from(0.12646669615856066488e-1_f64) * t23609 * t3327;
    let t31811 = F::cast_from(0.39837009289946609438e0_f64) * t31548 * t10546 * t4807;
    let t31825 = F::cast_from(0.85365019907028448797e-1_f64) * t23741 * t3347;
    let t31835 = F::cast_from(0.68292015925622759036e0_f64) * t2268 * t26938 * t6767;
    let t31838 = F::cast_from(0.68292015925622759036e0_f64) * t2268 * t7937 * t21389;
    (t31805, t31811, t31825, t31835, t31838)
}
