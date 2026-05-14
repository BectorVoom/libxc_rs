//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1440/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1440<F: Float>(t2526: F, t2858: F, t9788: F, t1048: F, t2850: F, t9573: F, t288: F, t2892: F, t97: F, t19720: F, t31444: F, t31446: F, t32980: F, t32982: F, t32984: F, t32985: F, t32986: F, t32987: F, t34858: F) -> (F, F, F, F) {
    let t34861 = 18.0 * t2858 * t9788 * t2526;
    let t34866 = 6.0 * t1048 * t9573 * t2850;
    let t34867 = t2892 * t288;
    let t34870 = 18.0 * t97 * t34867 * t2526;
    let t34871 = -t34858 - t34861 - t32980 - 0.14178e2 * t31444 - 0.14178e2 * t31446 - t34866 - t34870 - t32982 - t32984 + t32985 + t32986 + t32987 - t19720;
    (t34861, t34866, t34870, t34871)
}
