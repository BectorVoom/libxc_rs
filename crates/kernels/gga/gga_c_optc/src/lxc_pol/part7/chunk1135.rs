//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1135/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1135<F: Float>(t23685: F, t23651: F, t23653: F, t23655: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F, t23683: F) -> F {
    let t23686 = F::new(0.20068888888888888889e-1) * t23685;
    let t23687 = F::new(0.7389e-2) * t23651 - F::new(0.15510666666666666667e2) * t23653 + F::new(0.5170222222222222222e1) * t23655 + F::new(0.15510666666666666667e2) * t23660 - F::new(0.44334e-1) * t23664 + F::new(0.9852e-2) * t23667 + F::new(0.46531999999999999998e2) * t23670 - F::new(0.5170222222222222222e1) * t23673 - F::new(0.12925555555555555555e2) * t23676 - F::new(0.69798e2) * t23679 + t23683 + t23686;
    t23687
}
