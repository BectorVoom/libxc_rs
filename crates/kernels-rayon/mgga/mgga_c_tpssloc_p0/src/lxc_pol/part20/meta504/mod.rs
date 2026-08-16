//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta504(t9108: f64, t94: f64, t102: f64, t9174: f64, t12512: f64, t580: f64, t1404: f64, t3931: f64, t1395: f64, t3946: f64, t12537: f64, t576: f64, t2: f64, t591: f64, t21: f64, t9: f64, t587: f64, t598: f64, t14: f64, t2230: f64, t594: f64, t9223: f64, t22811: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t35577, t35761, t39022, t39024, t39026, t39028) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013(t9108, t94, t102, t9174, t12512, t580, t1404, t3931, t1395, t3946, t12537, t576);
        let (t39031, t39033, t39035, t39037, t39039, t39043) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2014(t2, t591, t21, t9, t587, t598, t14, t2230, t594, t9223, t22811, t19);
    (t35577, t35761, t39022, t39024, t39026, t39028, t39031, t39033, t39035, t39037, t39039, t39043)
}
