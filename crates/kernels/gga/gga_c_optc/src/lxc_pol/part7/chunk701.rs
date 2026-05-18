//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 701/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk701<F: Float>(t1924: F, t193: F, t1949: F, t197: F, t6560: F, t3575: F, t6653: F, t6656: F, t6660: F, t750: F, t201: F, t5: F) -> (F, F, F) {
    let t6663 = t193 * t1924 * t1949;
    let t6668 = t197 * t6560;
    let t6672 = t6653 - F::new(2200.0) / F::new(27.0) * t6656 + F::new(200.0) / F::new(9.0) * t6660 + F::new(200.0) / F::new(9.0) * t6663 - F::new(25.0) / F::new(3.0) * t193 * t3575 * t1949 - F::new(25.0) / F::new(9.0) * t193 * t750 * t6668;
    let t6674 = t5 * t6672 * t201;
    (t6668, t6672, t6674)
}
