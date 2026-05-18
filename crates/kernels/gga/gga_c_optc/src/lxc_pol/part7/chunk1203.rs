//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1203/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1203<F: Float>(t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23651: F, t23653: F, t23655: F, t23914: F) -> F {
    let t24855 = -F::new(0.91817777777777777776e0) * t23622 + F::new(0.68863333333333333332e0) * t23624 + F::new(0.76514814814814814814e0) * t23626 - F::new(0.15302962962962962963e1) * t23630 - F::new(0.516475e0) * t23633 + F::new(0.21424148148148148148e1) * t23635 - F::new(0.27545333333333333333e1) * t23637 + F::new(0.68863333333333333334e1) * t23640 + F::new(0.309885e1) * t23644 + F::new(0.250068e1) * t23647 + F::new(0.62517e0) * t23651 - F::new(0.6618234375e1) * t23914 - F::new(0.41318e1) * t23653 + F::new(0.13772666666666666666e1) * t23655;
    t24855
}
