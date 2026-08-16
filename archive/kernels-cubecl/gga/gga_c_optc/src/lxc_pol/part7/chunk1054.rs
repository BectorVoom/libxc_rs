//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1054/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1054<F: Float>(t2034: F, t22871: F, t1948: F, t6926: F, t2035: F, t6560: F, t2067: F, t6785: F, t162: F, t2017: F, t6956: F, t2003: F, t2010: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22872 = t2034 * t22871;
    let t22875 = t6926 * t1948;
    let t22876 = t2034 * t22875;
    let t22879 = t2035 * t6560;
    let t22880 = t2034 * t22879;
    let t22883 = t6785 * t2067;
    let t22884 = t162 * t22883;
    let t22887 = t6956 * t2017;
    let t22889 = t2003 * t2010;
    (t22872, t22875, t22876, t22879, t22880, t22883, t22884, t22887, t22889)
}
