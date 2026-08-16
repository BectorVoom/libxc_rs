//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1061/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1061<F: Float>(t2067: F, t616: F, t2035: F, t146: F, t6567: F, t671: F, t678: F, t2132: F, t7030: F, t155: F, t2078: F, t2157: F) -> (F, F, F, F) {
    let t23051 = t616 * t2067;
    let t23052 = t2035 * t23051;
    let t23065 = t146 * t671 * t6567;
    let t23066 = t23065 * t678;
    let t23068 = t7030 * t2132;
    let t23071 = t155 * t2157 * t2078;
    (t23052, t23066, t23068, t23071)
}
