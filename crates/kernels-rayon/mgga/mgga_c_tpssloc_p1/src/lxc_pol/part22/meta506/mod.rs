//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta506(t1100: f64, t21780: f64, t1661: f64, t5992: f64, t11265: f64, t21762: f64, t3297: f64, t136: f64, t1113: f64, t21769: f64, t21776: f64, t11219: f64, t21758: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21783, t21785, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1956(t1100, t21780, t1661, t5992, t11265, t21762, t3297, t136, t1113, t21769, t21776, t11219, t21758);
    (t21783, t21785, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801)
}
