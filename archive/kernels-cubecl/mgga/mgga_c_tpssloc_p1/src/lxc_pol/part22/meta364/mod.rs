//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1610;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1611;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta364<F: Float>(t5794: F, t950: F, t5791: F, t10556: F, t10832: F, t13563: F, t13598: F, t14409: F, t14410: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F, t10636: F, t14245: F, t14246: F, t291: F, t2932: F, t5790: F, t4471: F, t4475: F, t10632: F, t5774: F, t13727: F, t4359: F, t13520: F, t4400: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17451, t17454, t17471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1610::<F>(t5794, t950, t5791, t10556, t10832, t13563, t13598, t14409, t14410, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let t17488 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1611::<F>(t10556, t10636, t13563, t13598, t14245, t14246, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17490, t17492, t17493, t17496, t17499, t17500, t17504, t17506) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1612::<F>(t17488, t291, t2932, t5790, t950, t4471, t4475, t10632, t5774, t13727, t4359, t13520, t4400);
    (t17451, t17454, t17471, t17488, t17490, t17492, t17493, t17496, t17499, t17500, t17504, t17506)
}
