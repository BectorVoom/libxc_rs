//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1045/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1045<F: Float>(t23581: F, t23583: F, t23585: F, t23587: F, t23592: F, t23597: F, t23602: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F) -> (F, F) {
    let t23745 = -0.78666666666666666667e2 * t23581 - 0.52444444444444444446e3 * t23583 + 0.20977777777777777778e3 * t23585 - 0.12586666666666666667e4 * t23587 + 0.94399999999999999998e3 * t23592 - 0.10488888888888888889e3 * t23597 - 0.20977777777777777778e3 * t23602 - 17446.0 * t23605 - 0.14538333333333333333e4 * t23608 + 17446.0 * t23612 + 0.38768888888888888889e4 * t23614 + 0.58153333333333333332e4 * t23616;
    let t23758 = -0.19384444444444444445e4 * t23620 - 0.12922962962962962963e4 * t23622 + 0.96922222222222222224e3 * t23624 + 0.10769135802469135803e4 * t23626 - 0.21538271604938271605e4 * t23630 - 0.72691666666666666667e3 * t23633 + 0.30153580246913580247e4 * t23635 - 0.38768888888888888889e4 * t23637 + 0.96922222222222222221e4 * t23640 + 0.43614999999999999999e4 * t23644 + 1888.0 * t23647;
    (t23745, t23758)
}
