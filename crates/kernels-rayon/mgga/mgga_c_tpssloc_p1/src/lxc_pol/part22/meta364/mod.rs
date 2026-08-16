//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1610;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1611;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta364(t5794: f64, t950: f64, t5791: f64, t10556: f64, t10832: f64, t13563: f64, t13598: f64, t14409: f64, t14410: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64, t10636: f64, t14245: f64, t14246: f64, t291: f64, t2932: f64, t5790: f64, t4471: f64, t4475: f64, t10632: f64, t5774: f64, t13727: f64, t4359: f64, t13520: f64, t4400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17451, t17454, t17471) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1610(t5794, t950, t5791, t10556, t10832, t13563, t13598, t14409, t14410, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let t17488 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1611(t10556, t10636, t13563, t13598, t14245, t14246, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17490, t17492, t17493, t17496, t17499, t17500, t17504, t17506) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1612(t17488, t291, t2932, t5790, t950, t4471, t4475, t10632, t5774, t13727, t4359, t13520, t4400);
    (t17451, t17454, t17471, t17488, t17490, t17492, t17493, t17496, t17499, t17500, t17504, t17506)
}
