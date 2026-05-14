//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk861<F: Float>(t17263: F, t17275: F, t1426: F, t4835: F, t4846: F, t10348: F, t13649: F, t13651: F, t13653: F, t16650: F, t16747: F, t16750: F, t16763: F, t16766: F, t8362: F, t8364: F) -> (F, F, F, F) {
    let t17276 = t17263 + t17275;
    let t17284 = t4835 * t1426;
    let t17287 = t1426 * t4846;
    let t17299 = 0.52444444444444444444e2 * t13649 - 0.31466666666666666667e3 * t13651 + 0.15733333333333333334e3 * t13653 - t8362 - 0.72691666666666666667e3 * t16650 - t8364 - 0.47199999999999999999e3 * t16747 + 0.47199999999999999999e3 * t16763 + 0.15733333333333333333e3 * t16750 - 0.78666666666666666666e2 * t16766 - 0.26222222222222222223e3 * t10348;
    (t17276, t17284, t17287, t17299)
}
