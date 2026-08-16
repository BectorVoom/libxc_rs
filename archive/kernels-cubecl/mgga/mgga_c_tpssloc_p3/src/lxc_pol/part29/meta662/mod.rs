//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2202;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta662<F: Float>(t22633: F, t22635: F, t26214: F, t3719: F, t225: F, t26219: F, t1985: F, t7700: F, t80707: F, t214: F, t5318: F, t6888: F, t6891: F, t81311: F, t16065: F, t1992: F, t22897: F, t26378: F, t6914: F, t16044: F, t6976: F, t1372: F, t1799: F, t1307: F, t26331: F, t26446: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90728, t90732, t90737, t90739, t90741) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2202::<F>(t22633, t22635, t26214, t3719, t225, t26219, t1985, t7700, t80707, t214, t5318, t6888, t6891);
        let (t90743, t90747, t90750, t90752, t90754, t90757) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2203::<F>(t81311, t16065, t1992, t22897, t26378, t6914, t16044, t6976, t1372, t1799, t1307, t26331, t26446);
    (t90728, t90732, t90737, t90739, t90741, t90743, t90747, t90750, t90752, t90754, t90757)
}
