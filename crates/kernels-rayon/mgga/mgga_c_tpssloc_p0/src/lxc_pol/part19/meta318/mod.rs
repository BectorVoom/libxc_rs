//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1130;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta318(t39514: f64, t677: f64, t9919: f64, t3684: f64, t2393: f64, t2535: f64, t12110: f64, t9882: f64, t12466: f64, t3719: f64, t3918: f64, t39483: f64, t39490: f64, t39492: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39511: f64, t39513: f64, t2420: f64, t701: f64, t9778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39515, t39516, t39518, t39519, t39521, t39523, t39524) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1130(t39514, t677, t9919, t3684, t2393, t2535, t12110, t9882, t12466, t3719, t3918, t39483, t39490, t39492, t39496, t39499, t39502, t39505, t39508, t39511, t39513);
        let t39529 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1131(t2420, t701, t9778);
    (t39515, t39516, t39518, t39519, t39521, t39523, t39524, t39529)
}
