//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2137;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta501(t10556: f64, t10608: f64, t13598: f64, t14352: f64, t14353: f64, t14354: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64, t324: f64, t300: f64, t5689: f64, t892: f64, t914: f64, t11094: f64, t5950: f64, t3216: f64, t5946: f64, t4483: f64, t4498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t17191 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2137(t10556, t10608, t13598, t14352, t14353, t14354, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17192, t17194, t17195, t17197, t17198, t17202, t17209) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2138(t17191, t324, t300, t5689, t892, t914, t11094, t5950, t3216, t5946, t4483, t4498);
    (t17191, t17192, t17194, t17195, t17197, t17198, t17202, t17209)
}
