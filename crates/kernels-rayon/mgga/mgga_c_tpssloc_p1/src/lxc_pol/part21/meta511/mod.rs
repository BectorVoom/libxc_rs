//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2157;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta511(t17375: f64, t17449: f64, t17516: f64, t17558: f64, t300: f64, t2940: f64, t5808: f64, t10629: f64, t5774: f64, t10632: f64, t950: f64, t959: f64, t225: f64, t5849: f64, t1603: f64, t4657: f64, t1634: f64, t4693: f64, t3174: f64, t5851: f64, t17183: f64, t977: f64, t17178: f64, t2979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17561, t17563, t17564, t17566, t17568) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2157(t17375, t17449, t17516, t17558, t300, t2940, t5808, t10629, t5774, t10632, t950, t959);
        let (t17575, t17579, t17583, t17588, t17593, t17596) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2158(t225, t5849, t1603, t4657, t1634, t4693, t3174, t5851, t17183, t977, t17178, t2979);
    (t17561, t17563, t17564, t17566, t17568, t17575, t17579, t17583, t17588, t17593, t17596)
}
