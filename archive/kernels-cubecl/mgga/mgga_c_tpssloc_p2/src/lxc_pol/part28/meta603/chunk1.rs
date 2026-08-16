//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1908/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1908<F: Float>(t22635: F, t26214: F, t26331: F, t3734: F, t1985: F, t22666: F, t26202: F, t22642: F, t22643: F, t7700: F, t22674: F, t6897: F) -> (F, F, F, F) {
    let t90634 = t26331 * t22635 * t26214 * t3734;
    let t90639 = t1985 * t22666 * t26202;
    let t90642 = t22642 * t22643 * t7700;
    let t90645 = t6897 * t22674 * t26202;
    (t90634, t90639, t90642, t90645)
}
