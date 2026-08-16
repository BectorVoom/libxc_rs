//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 722/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk722<F: Float>(t6878: F, t6879: F, t161: F, t2024: F, t127: F, t136: F, t2079: F, t634: F, t648: F, t108: F, t6567: F, t117: F, t56: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6880 = t6878 * t6879;
    let t6881 = t161 * t6880;
    let t6884 = t6878 * t2024;
    let t6885 = t161 * t6884;
    let t6888 = t6878 * t127;
    let t6889 = t161 * t6888;
    let t6892 = t2079 * t136;
    let t6893 = t634 * t6892;
    let t6894 = t6893 * t648;
    let t6896 = t108 * t6567;
    let t6899 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t6896 * t56 * t117;
    (t6880, t6881, t6884, t6885, t6888, t6889, t6892, t6893, t6894, t6896, t6899)
}
