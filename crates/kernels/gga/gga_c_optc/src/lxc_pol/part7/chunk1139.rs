//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1139/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1139<F: Float>(t23685: F, t23651: F, t23653: F, t23655: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F, t23769: F) -> F {
    let t23770 = F::new(0.12819753086419753086e4) * t23685;
    let t23771 = F::new(0.47199999999999999999e3) * t23651 - F::new(0.58153333333333333333e4) * t23653 + F::new(0.19384444444444444445e4) * t23655 + F::new(0.58153333333333333332e4) * t23660 - F::new(2832.0) * t23664 + F::new(0.62933333333333333332e3) * t23667 + F::new(17446.0) * t23670 - F::new(0.19384444444444444444e4) * t23673 - F::new(0.4846111111111111111e4) * t23676 - F::new(26169.0) * t23679 + t23769 + t23770;
    t23771
}
