//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1264/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1264<F: Float>(t26314: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F, t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26319: F, t26324: F, t26388: F, t26394: F, t26396: F) -> (F, F) {
    let t27972 = -0.21538271604938271605e4 * t26339 - 0.72691666666666666667e3 * t26343 - 0.52444444444444444446e3 * t26363 - 0.17481481481481481482e3 * t26365 + 0.20977777777777777778e3 * t26367 + 0.932345679012345679e2 * t26369 + 0.96922222222222222224e3 * t26314 + 0.10488888888888888889e4 * t26372 - 0.81580246913580246914e2 * t26376 - 0.78666666666666666667e2 * t26379 - 0.20977777777777777778e3 * t26382 + 0.62933333333333333332e3 * t26385;
    let t27985 = -0.10488888888888888889e3 * t26388 + 0.58153333333333333332e4 * t26319 - 0.19384444444444444444e4 * t26324 - 0.12586666666666666667e4 * t26394 + 0.20977777777777777778e3 * t26396 + 0.58153333333333333332e4 * t26280 - 17446.0 * t26284 - 0.14538333333333333333e4 * t26293 + 17446.0 * t26296 + 0.43614999999999999999e4 * t26304 - 0.38768888888888888889e4 * t26311;
    (t27972, t27985)
}
