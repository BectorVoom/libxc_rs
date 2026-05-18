//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1138/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1138<F: Float>(t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23682: F) -> (F, F) {
    let t23758 = -F::new(0.19384444444444444445e4) * t23620 - F::new(0.12922962962962962963e4) * t23622 + F::new(0.96922222222222222224e3) * t23624 + F::new(0.10769135802469135803e4) * t23626 - F::new(0.21538271604938271605e4) * t23630 - F::new(0.72691666666666666667e3) * t23633 + F::new(0.30153580246913580247e4) * t23635 - F::new(0.38768888888888888889e4) * t23637 + F::new(0.96922222222222222221e4) * t23640 + F::new(0.43614999999999999999e4) * t23644 + F::new(1888.0) * t23647;
    let t23769 = F::new(0.75383950617283950617e4) * t23682;
    (t23758, t23769)
}
